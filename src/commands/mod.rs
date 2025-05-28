pub mod effect;

trait Command {
    fn into_mcfunction(self) -> String;
}
