use crate::error;

pub fn add(name: &str, url: &str) -> Result<(), error::OngakuError> {
    println!("TODO in add command");
    Ok(())
}

pub fn sync(verify_: bool) -> Result<(), error::OngakuError> {
    if verify_ {
        verify()?;
    };
    println!("TODO in sync command");
    Ok(())
}

pub fn verify() -> Result<(), error::OngakuError> {
    println!("TODO in verify command");
    Ok(())
}
