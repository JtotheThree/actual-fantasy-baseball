struct RoleGuard {
    role: Role,
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // TODO: auth disabling is needed for tests. try to reimplement when https://github.com/rust-lang/rust/issues/45599 will be resolved (using cfg(test))
        if let Ok(boolean) = env::var("DISABLE_AUTH") {
            let disable_auth = bool::from_str(boolean.as_str()).expect("Can't parse bool");
            if disable_auth {
                return Ok(());
            }
        };
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}