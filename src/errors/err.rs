use anyhow::Result;

fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(7);
    }

    return Ok(());
}

fn anyhow_error_me(throw: bool) -> Result<()> {
    if throw {
        return Err(anyhow!("this should never be true "));
    }

    std::fs::read(PathBuf::from("./foo")).context("something went wrong")?;

    return Ok(());
}

fn main() -> Result<(), usize> {
     error_me(false)?;

     if error_me(true).is_ok() {} 

     return Ok(());
}