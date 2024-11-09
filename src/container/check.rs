use const_format::formatcp;

use crate::error::Error;

pub trait CheckContainerFit {
    const IS_FIT: ();
}

macro_rules! check_failed {
    ($exceed: literal) => {
        panic!(
            "{}",
            formatcp!(
                "\n\
                ------------------------------------------------------------------\n\
                cannot create `StackBox`: \n\
                the {} of the value is larger than that of the container. \n\
                please consider make a larger container.\n\
                ------------------------------------------------------------------\n",
                $exceed
            )
        )
    };
}

impl<T, Ctnr> CheckContainerFit for (T, Ctnr) {
    const IS_FIT: () = {
        match check_container_fit::<T, Ctnr>() {
            Ok(()) => (),
            Err(Error::SizeTooLarge) => check_failed!("SIZE"),
            Err(Error::AlignTooLarge) => check_failed!("ALIGN"),
        }
    };
}

pub const fn check_container_fit<T, Ctnr>() -> Result<(), Error> {
    let size_of_t = size_of::<T>();
    let size_of_container = size_of::<Ctnr>();
    let align_of_t = align_of::<T>();
    let align_of_container = align_of::<Ctnr>();

    if size_of_t > size_of_container {
        return Err(Error::SizeTooLarge);
    }

    if align_of_t > align_of_container {
        return Err(Error::AlignTooLarge);
    }

    return Ok(());
}
