use bindings::ffi;
use bindings::AsNative;
use bindings::{c_void, c_bool};
use random::Rng;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug)]
pub enum TraverseOrder {
    PreOrder,
    InOrder,
    PostOrder,
    LevelOrder,
    InvertedLevelOrder,
}

pub struct BSP {
    bsp: *mut ffi::TCOD_bsp_t,
    root: bool
}

impl Deref for BSP {
    type Target = ffi::TCOD_bsp_t;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.bsp }
    }
}

impl DerefMut for BSP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.bsp }
    }
}

extern "C" fn callback_wrapper<T>(node: *mut ffi::TCOD_bsp_t, user_data: *mut c_void) -> c_bool
    where T: FnMut(&BSP) -> bool
{
    let callback_ptr = user_data as *mut T;
    let cb: &mut T = unsafe {
        &mut *callback_ptr
    };
    let bsp = BSP { bsp: node, root: false };
    cb(&bsp) as c_bool
}

impl BSP {
    pub fn new_with_size(x: i32, y: i32, w: i32, h: i32) -> Self {
        let bsp = unsafe {
            ffi::TCOD_bsp_new_with_size(x, y, w, h)
        };
        BSP { bsp: bsp, root: true }
    }

    pub fn remove_sons(&self) {
        unsafe { ffi::TCOD_bsp_remove_sons(self.bsp) }
    }

    pub fn split_once(&self, horizontal: bool, position: i32) {
        unsafe { ffi::TCOD_bsp_split_once(self.bsp, horizontal as u8, position) }
    }

    pub fn split_recursive(&self,
                           randomizer: Option<Rng>,
                           nb: i32,
                           min_h_size: i32,
                           min_v_size: i32,
                           max_h_ratio: f32,
                           max_v_ratio: f32) {
        let rnd = randomizer.unwrap_or(Rng::get_instance());
        unsafe {
            ffi::TCOD_bsp_split_recursive(self.bsp,
                                          *rnd.as_native(),
                                          nb,
                                          min_h_size,
                                          min_v_size,
                                          max_h_ratio,
                                          max_v_ratio)
        }
    }

    pub fn resize(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::TCOD_bsp_resize(self.bsp, x, y, w, h) }
    }

    pub fn left(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_left(self.bsp) },
            root: false
        }
    }

    pub fn right(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_right(self.bsp) },
            root:false
        }
    }

    pub fn father(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_father(self.bsp) },
            root: false,
        }
    }

    pub fn is_leaf(&self) -> bool {
        unsafe { ffi::TCOD_bsp_is_leaf(self.bsp) != 0 }
    }

    pub fn contains(&self, cx: i32, cy: i32) -> bool {
        unsafe { ffi::TCOD_bsp_contains(self.bsp, cx, cy) != 0 }
    }

    pub fn find_node(&self, cx: i32, cy: i32) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_find_node(self.bsp, cx, cy) },
            root: false
        }
    }

    pub fn horizontal(&self) -> bool {
        self.horizontal != 0
    }

    pub fn set_horizontal(&mut self, h: bool) {
        self.horizontal = h as u8;
    }

    pub fn traverse<F>(&self, order: TraverseOrder, mut callback: F) -> bool
        where F: FnMut(&BSP) -> bool
    {
        let cb: &mut FnMut(&BSP) -> bool = &mut callback;
        let retval = unsafe {
            match order {
                TraverseOrder::PreOrder =>
                    ffi::TCOD_bsp_traverse_pre_order(self.bsp,
                                                     Some(callback_wrapper::<F>),
                                                     cb as *mut _ as *mut c_void),
                TraverseOrder::InOrder =>
                    ffi::TCOD_bsp_traverse_in_order(self.bsp,
                                                    Some(callback_wrapper::<F>),
                                                    cb as *mut _ as *mut c_void),
                TraverseOrder::PostOrder =>
                    ffi::TCOD_bsp_traverse_post_order(self.bsp,
                                                      Some(callback_wrapper::<F>),
                                                      cb as *mut _ as *mut c_void),
                TraverseOrder::LevelOrder =>
                    ffi::TCOD_bsp_traverse_level_order(self.bsp,
                                                       Some(callback_wrapper::<F>),
                                                       cb as *mut _ as *mut c_void),
                TraverseOrder::InvertedLevelOrder =>
                    ffi::TCOD_bsp_traverse_inverted_level_order(self.bsp,
                                                                Some(callback_wrapper::<F>),
                                                                cb as *mut _ as *mut c_void),
            }
        };
        retval != 0
    }
}

impl Drop for BSP {
    fn drop(&mut self) {
        if self.root {
            unsafe { ffi::TCOD_bsp_delete(self.bsp) }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BSP;
    use super::TraverseOrder;

    #[test]
    #[allow(unused_variables)]
    fn created_destroyed_no_panic() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);
        let left = bsp.left(); // left has null .bsp
    }

    #[test]
    fn accessors() {
        let mut bsp = BSP::new_with_size(0, 0, 50, 60);

        assert_eq!(bsp.x, 0);
        assert_eq!(bsp.y, 0);
        assert_eq!(bsp.w, 50);
        assert_eq!(bsp.h, 60);
        assert_eq!(bsp.horizontal(), false);
        bsp.x = 10;
        bsp.y = 20;
        bsp.set_horizontal(true);
        assert_eq!(bsp.x, 10);
        assert_eq!(bsp.y, 20);
        assert_eq!(bsp.horizontal(), true);
    }

    #[test]
    fn split() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);

        assert_eq!(bsp.position, 0);
        assert_eq!(bsp.horizontal(), false);

        bsp.split_once(true, 20);
        assert_eq!(bsp.position, 20);
        assert_eq!(bsp.horizontal(), true);
    }

    #[test]
    fn split_recursive() {
        let bsp = BSP::new_with_size(0, 0, 100,100);
        let mut counter = 0;

        bsp.split_recursive(None, 2, 5, 5, 1.5, 1.5);
        bsp.traverse(TraverseOrder::PreOrder, |node| {
            assert!(node.h >= 5);
            assert!(node.w >= 5);
            counter += 1;
            true
        });
        assert_eq!(counter, 7);
    }

    #[test]
    fn children() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);

        assert!(bsp.left().bsp.is_null());
        assert_eq!(bsp.level, 0);

        bsp.split_once(false, 20);
        assert!(!bsp.left().bsp.is_null());
        assert!(!bsp.right().bsp.is_null());
        assert_eq!(bsp.left().level, 1);
        assert_eq!(bsp.right().level, 1);
    }

    #[test]
    fn father() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);
        assert!(bsp.father().bsp.is_null());

        bsp.split_once(false, 30);
        assert!(!bsp.left().father().bsp.is_null());
        assert!(!bsp.right().father().bsp.is_null());
    }

    #[test]
    fn traverse() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);
        let mut counter = 0;

        bsp.traverse(TraverseOrder::PreOrder, |node| {
            assert_eq!(node.bsp, bsp.bsp);
            counter += 1;
            true
        });
        assert_eq!(counter, 1);

        counter = 0;
        bsp.split_once(true, 25);
        bsp.traverse(TraverseOrder::PreOrder, |_node| {
            counter += 1;
            true
        });
        assert_eq!(counter, 3);
    }

    #[test]
    fn traverse_orders() {
        let root = BSP::new_with_size(0, 0, 100,100);
        let mut counter = 0;

        root.split_recursive(None, 2, 5, 5, 1.5, 1.5);

        let middle_left = root.left();
        let middle_right = root.right();
        let leaf1 = middle_left.left();
        let leaf2 = middle_left.right();
        let leaf3 = middle_right.left();
        let leaf4 = middle_right.right();

        root.traverse(TraverseOrder::PreOrder, |node| {
            match counter {
                0 => assert_eq!(node.bsp, root.bsp),
                1 => assert_eq!(node.bsp, middle_left.bsp),
                2 => assert_eq!(node.bsp, leaf1.bsp),
                3 => assert_eq!(node.bsp, leaf2.bsp),
                4 => assert_eq!(node.bsp, middle_right.bsp),
                5 => assert_eq!(node.bsp, leaf3.bsp),
                6 => assert_eq!(node.bsp, leaf4.bsp),
                _ => panic!("Wrong number of nodes in the tree"),
            };
            counter += 1;
            true
        });

        counter = 0;
        root.traverse(TraverseOrder::InOrder, |node| {
            match counter {
                0 => assert_eq!(node.bsp, leaf1.bsp),
                1 => assert_eq!(node.bsp, middle_left.bsp),
                2 => assert_eq!(node.bsp, leaf2.bsp),
                3 => assert_eq!(node.bsp, root.bsp),
                4 => assert_eq!(node.bsp, leaf3.bsp),
                5 => assert_eq!(node.bsp, middle_right.bsp),
                6 => assert_eq!(node.bsp, leaf4.bsp),
                _ => panic!("Wrong number of nodes in the tree"),
            };
            counter += 1;
            true
        });

        counter = 0;
        root.traverse(TraverseOrder::PostOrder, |node| {
            match counter {
                0 => assert_eq!(node.bsp, leaf1.bsp),
                1 => assert_eq!(node.bsp, leaf2.bsp),
                2 => assert_eq!(node.bsp, middle_left.bsp),
                3 => assert_eq!(node.bsp, leaf3.bsp),
                4 => assert_eq!(node.bsp, leaf4.bsp),
                5 => assert_eq!(node.bsp, middle_right.bsp),
                6 => assert_eq!(node.bsp, root.bsp),
                _ => panic!("Wrong number of nodes in the tree"),
            };
            counter += 1;
            true
        });

        counter = 0;
        root.traverse(TraverseOrder::LevelOrder, |node| {
            match counter {
                0 => assert_eq!(node.bsp, root.bsp),
                1 => assert_eq!(node.bsp, middle_left.bsp),
                2 => assert_eq!(node.bsp, middle_right.bsp),
                3 => assert_eq!(node.bsp, leaf1.bsp),
                4 => assert_eq!(node.bsp, leaf2.bsp),
                5 => assert_eq!(node.bsp, leaf3.bsp),
                6 => assert_eq!(node.bsp, leaf4.bsp),
                _ => panic!("Wrong number of nodes in the tree"),
            };
            counter += 1;
            true
        });

        counter = 0;
        root.traverse(TraverseOrder::InvertedLevelOrder, |node| {
            match counter {
                0 => assert_eq!(node.bsp, leaf4.bsp),
                1 => assert_eq!(node.bsp, leaf3.bsp),
                2 => assert_eq!(node.bsp, leaf2.bsp),
                3 => assert_eq!(node.bsp, leaf1.bsp),
                4 => assert_eq!(node.bsp, middle_right.bsp),
                5 => assert_eq!(node.bsp, middle_left.bsp),
                6 => assert_eq!(node.bsp, root.bsp),
                _ => panic!("Wrong number of nodes in the tree"),
            };
            counter += 1;
            true
        });
    }

    #[test]
    fn break_traverse() {
        let bsp = BSP::new_with_size(0, 0, 100,100);
        let mut counter = 0;

        bsp.split_recursive(None, 2, 5, 5, 1.5, 1.5);
        bsp.traverse(TraverseOrder::PreOrder, |node| {
            counter += 1;
            false
        });
        assert_eq!(counter, 1);
    }
}
