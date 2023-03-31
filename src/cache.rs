use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};

pub struct LruCache<K, V> {
    head: Rc<RefCell<Node<K, V>>>,
    tail: Rc<RefCell<Node<K, V>>>,
    map: HashMap<RcWrap<K>, Rc<RefCell<Node<K, V>>>>,
    capacity: NonZeroUsize,
}

impl<K, V> LruCache<K, V>
where
    K: Hash + Eq,
{
    pub fn new(capacity: NonZeroUsize) -> Self {
        let head = new_empty_node();
        let tail = new_empty_node();
        connect_two(&head, &tail);
        Self {
            head,
            tail,
            map: HashMap::new(),
            capacity,
        }
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }

    pub fn capacity(&self) -> NonZeroUsize {
        self.capacity
    }

    pub fn get<Q>(&mut self, key: &Q) -> Option<Ref<V>>
    where
        RcWrap<K>: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let node = self.map.get(key)?;
        detach(node);
        insert_after(node, &self.head);
        Ref::filter_map(RefCell::borrow(node), |n| n.value.as_ref()).ok()
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.map.get(&key) {
            detach(node);
            insert_after(node, &self.head);
            node.borrow_mut().value = Some(value);
            return;
        }

        if self.size() >= self.capacity().into() {
            let last = remove_prev(&self.tail);
            if let Some(last) = &last {
                if let Some(key) = &RefCell::borrow(last).key {
                    self.map.remove(key);
                }
            }
        }

        let key = Rc::new(key);
        let node = new_data_node(&key, value);
        insert_after(&node, &self.head);
        self.map.insert(RcWrap(key), node);
    }
}

struct Node<K, V> {
    key: Option<Rc<K>>,
    value: Option<V>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
    prev: Weak<RefCell<Node<K, V>>>,
}

fn new_empty_node<K, V>() -> Rc<RefCell<Node<K, V>>> {
    Rc::new(RefCell::new(Node {
        key: None,
        value: None,
        next: None,
        prev: Weak::new(),
    }))
}

fn new_data_node<K, V>(key: &Rc<K>, value: V) -> Rc<RefCell<Node<K, V>>> {
    Rc::new(RefCell::new(Node {
        key: Some(Rc::clone(key)),
        value: Some(value),
        next: None,
        prev: Weak::new(),
    }))
}

fn connect_two<K, V>(left: &Rc<RefCell<Node<K, V>>>, right: &Rc<RefCell<Node<K, V>>>) {
    left.borrow_mut().next = Some(Rc::clone(right));
    right.borrow_mut().prev = Rc::downgrade(left);
}

fn detach<K, V>(node: &Rc<RefCell<Node<K, V>>>) {
    let node = RefCell::borrow(node);
    let prev = &node.prev.upgrade();
    let next = &node.next;
    if let (Some(prev), Some(next)) = (prev, next) {
        prev.borrow_mut().next = Some(Rc::clone(next));
        next.borrow_mut().prev = Rc::downgrade(prev);
    }
}

fn insert_after<K, V>(node: &Rc<RefCell<Node<K, V>>>, after: &Rc<RefCell<Node<K, V>>>) {
    if let Some(next) = &after.borrow_mut().next {
        node.borrow_mut().next = Some(Rc::clone(next));
        next.borrow_mut().prev = Rc::downgrade(node);
    }
    after.borrow_mut().next = Some(Rc::clone(node));
    node.borrow_mut().prev = Rc::downgrade(after);
}

fn remove_prev<K, V>(node: &Rc<RefCell<Node<K, V>>>) -> Option<Rc<RefCell<Node<K, V>>>> {
    let prev = &RefCell::borrow(node).prev.upgrade()?;
    if let Some(left) = &RefCell::borrow(prev).prev.upgrade() {
        left.borrow_mut().next = Some(Rc::clone(node));
        node.borrow_mut().prev = Rc::downgrade(left);
    }
    Some(Rc::clone(prev))
}

// wrapper type for Rc because Rc does not have useful Borrow implementations
// such as: impl Borrow<str> for Rc<String>
// This makes Rc difficult to use as a HashMap key.
#[derive(Hash, PartialEq, Eq)]
pub struct RcWrap<T>(Rc<T>);

impl<T> Borrow<T> for RcWrap<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> Borrow<Rc<T>> for RcWrap<T> {
    fn borrow(&self) -> &Rc<T> {
        &self.0
    }
}

// Useful Borrow implementations for RcWrap so that users can pass in the convenient barrowed type
// in the cache's get method. I would prefer to have just one implementation like this:
//
// impl<T, U> Borrow<U> for RcWrap<T> where T: Borrow<U>
//
// instead of having different implementations for each type, but Rust currently does not allow
// this due to conflicting implementations. Hopefully it will be possible in the future with
// specializations.
impl Borrow<str> for RcWrap<String> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Borrow<OsStr> for RcWrap<OsString> {
    fn borrow(&self) -> &OsStr {
        &self.0
    }
}

impl Borrow<Path> for RcWrap<PathBuf> {
    fn borrow(&self) -> &Path {
        &self.0
    }
}

impl<T> Borrow<[T]> for RcWrap<Vec<T>> {
    fn borrow(&self) -> &[T] {
        &self.0
    }
}
