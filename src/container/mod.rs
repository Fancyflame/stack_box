pub(crate) mod check;

macro_rules! make_align {
    ($($AlignN:ident $N:literal,)*) => {
        $(
            #[repr(align($N))]
            #[derive(Clone, Copy)]
            pub struct $AlignN<const SIZE: usize>([u8; SIZE]);

            impl<const SIZE: usize> ContainerCalculateHelper
                for ContainerCalculator<$N, SIZE>
            {
                type Result = $AlignN<SIZE>;
            }

            impl<const SIZE:usize> ContainerSealed for $AlignN<SIZE> {}
            impl<const SIZE:usize> StackBoxContainer for $AlignN<SIZE> {}
        )*
    };
}

make_align!(
    Align1  1,
    Align2  2,
    Align4  4,
    Align8  8,
    Align16 16,
    Align32 32,
    Align64 64,
);

pub struct ContainerCalculator<const ALIGN: usize, const SIZE: usize>;

pub trait ContainerCalculateHelper {
    type Result: ContainerSealed;
}

pub trait ContainerSealed {}

pub trait StackBoxContainer: ContainerSealed {}

/// Given align and size, it will returns the container exactly can put
/// the data in.
pub type CalculateContainer<const ALIGN: usize, const SIZE: usize> =
    <ContainerCalculator<ALIGN, SIZE> as ContainerCalculateHelper>::Result;
