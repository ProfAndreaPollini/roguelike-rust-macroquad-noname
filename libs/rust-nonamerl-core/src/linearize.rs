use morton_encoding::morton_decode;

use crate::Scalar;

pub trait EncodeMorton: From<Self::Morton> + Into<Self::Morton> {
    /// The Morton code (Z order) for this vector.
    type Morton;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Morton2<T: Scalar>(T);

impl<T: Scalar>  Morton2<T> {
    pub fn translate_fn(x: T) -> T {
        
        x
    }

    pub fn untranslate_fn(x: T) -> T {
        x
    }
}



impl<T:Scalar> From<Morton2<T>> for [T; 2] {
    #[inline]
    fn from(m: Morton2<T>) -> Self { 
        let [y, x] = morton_decode(m.0);
        [Morton2::<T>::untranslate_fn(x), Morton2::<T>::untranslate_fn(y)]
    }
} 
impl<T:Scalar> From<[T; 2]> for Morton2<T> {
    #[inline]
    fn from([x, y]: [T; 2]) -> Morton2<T> {

        

        Morton2<T>(morton_encode([Morton2<T>::translate_fn(y), translate_fn(x)]))
    }

   
}
