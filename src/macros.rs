#[macro_export]
macro_rules! FitStackBox {
    ($T: ty, $Fit: ty) => {
        $crate::StackBox<
            $T,
            $crate::FitContainer!($Fit)
        >
    };
}

#[macro_export]
macro_rules! FitContainer {
    ($Fit: ty) => {
        $crate::CalculateContainer<
            {::core::mem::align_of::<$Fit>()},
            {::core::mem::size_of::<$Fit>()},
        >
    };
}

#[macro_export]
macro_rules! coerce {
    ($sb:expr) => {
        $crate::StackBox::coerce_unsized($sb, |checker| checker as &mut _)
    };
}
