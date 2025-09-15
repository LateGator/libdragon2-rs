macro_rules! wrapper_ref {
    ($ident:ident$(<$($lt:lifetime),+ $(, $ty:ident)* $(,)?>)? => $target:ty { $self:ident => $path:expr }) => {
        impl $(<$($lt),+ $(,$ty)*>)? ::core::ops::Deref for $ident $(<$($lt),+ $(,$ty)*>)? {
            type Target = $target;
            #[inline] fn deref(&$self) -> &$target { $path }
        }
        impl $(<$($lt),+ $(,$ty)*>)? ::core::borrow::Borrow<$target> for $ident $(<$($lt),+ $(,$ty)*>)? {
            #[inline] fn borrow(&$self) -> &$target { $path }
        }
        impl $(<$($lt),+ $(,$ty)*>)? ::core::convert::AsRef<$target> for $ident $(<$($lt),+ $(,$ty)*>)? {
            #[inline] fn as_ref(&$self) -> &$target { $path }
        }
    };
}

macro_rules! wrapper_mut {
    ($ident:ident $(<$($lt:lifetime),+ $(, $ty:ident)* $(,)?>)? => $target:ty { $self:ident => $path:expr }) => {
        impl $(<$($lt),+ $(,$ty)*>)? ::core::ops::DerefMut for $ident $(<$($lt),+ $(,$ty)*>)? {
            #[inline] fn deref_mut(&mut $self) -> &mut $target { $path }
        }
        impl $(<$($lt),+ $(,$ty)*>)? ::core::borrow::BorrowMut<$target> for $ident $(<$($lt),+ $(,$ty)*>)? {
            #[inline] fn borrow_mut(&mut $self) -> &mut $target { $path }
        }
        impl $(<$($lt),+ $(,$ty)*>)? ::core::convert::AsMut<$target> for $ident $(<$($lt),+ $(,$ty)*>)? {
            #[inline] fn as_mut(&mut $self) -> &mut $target { $path }
        }
    };
}

macro_rules! wrapper {
    ($ident:ident $(<$($lt:lifetime),+ $(, $ty:ident)* $(,)?>)? => $target:ty { $self:ident => $path:expr }) => {
        wrapper_ref! { $ident $(<$($lt),+ $(, $ty)*>)? => $target { $self => &$path } }
        wrapper_mut! { $ident $(<$($lt),+ $(, $ty)*>)? => $target { $self => &mut $path } }
    };
}

macro_rules! ptr_wrapper {
    ($ident:ident $(<$($lt:lifetime),+ $(, $ty:ident)* $(,)?>)? => $target:ty { $self:ident => $path:expr }) => {
        wrapper_ref! { $ident $(<$($lt),+ $(, $ty)*>)? => $target { $self => unsafe { $path.as_ref() } } }
        wrapper_mut! { $ident $(<$($lt),+ $(, $ty)*>)? => $target { $self => unsafe { $path.as_mut() } } }
    };
}

macro_rules! static_wrapper {
    ($ident:ident $(<$($lt:lifetime),+ $(, $ty:ident)* $(,)?>)? => $target:ty { $path:expr }) => {
        wrapper_ref! { $ident $(<$($lt),+ $(, $ty)*>)? => $target { self => unsafe { core::mem::transmute(&$path) } } }
        wrapper_mut! { $ident $(<$($lt),+ $(, $ty)*>)? => $target { self => unsafe { core::mem::transmute(&mut $path) } } }
    };
}
