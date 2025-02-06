use tilc_ast::{Token, TokenKind};
use tilc_errors::{Diag, DiagCtxtHandle, Diagnostic, Level};

pub struct ExpectedToken {
  pub expected_token_kind: TokenKind,
  pub current_token: Token,
}
impl<'a> Diagnostic<'a> for ExpectedToken {
  fn into_diag(self, dcx: DiagCtxtHandle<'a>, level: Level) -> Diag<'a> {
    let message: String = format!(
      "Unexpected token {{{:?}}} expected token kind {{{:?}}}",
      self.current_token, self.expected_token_kind
    );
    let mut diag: Diag = Diag::new(dcx, level, message);
    diag.span(self.current_token.span);

    return diag;
  }
}
