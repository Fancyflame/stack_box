#[macro_export]
macro_rules! FitStackBox {
    ($T: ty, $Fit: ty) => {
        $crate::StackBox<
            $T,
            $crate::CalculateContainer<
                {::core::mem::align_of::<$Fit>()},
                {::core::mem::size_of::<$Fit>()},
            >
        >
    };
}

#[macro_export]
macro_rules! coerce {
    ($sb:expr, $Target: ty) => {
        $crate::StackBox::coerce_unsized($sb, |checker| {
            (checker as &mut $crate::UnsizeChecker<$Target, _>).set_checked()
        })
    };
}
