
/// This function swaps between two objects, that implement [`Clone`]`, of the same type.
/// 
/// # Examples
/// ```
/// use crate::granger_things::swap;
/// 
/// let mut x = 10;
/// let mut y = 100;
/// swap(&mut x, &mut y);
/// assert_eq!(x, 100);
/// assert_eq!(y, 10);
/// 
/// let mut mes_1 = "hello";
/// let mut mes_2 = "world";
/// swap(&mut mes_1, &mut mes_2);
/// assert_eq!(mes_1, "world");
/// assert_eq!(mes_2, "hello");
/// ```
pub fn swap<T: Clone>(obj1: &mut T, obj2: &mut T) {
    let temp = obj1.clone();
    *obj1 = obj2.clone();
    *obj2 = temp;
}