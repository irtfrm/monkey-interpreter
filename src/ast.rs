use crate::token;

trait Node {
    fn token_literal(&self) -> &str;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return "";
        }
    }
}

struct LetStatement {
    token: token::Token,
    name: Identifier,
    value: dyn Expression,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str { return self.token.get_literal();}
}

struct Identifier {
    token: token::Token,
    value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> &str { return self.token.get_literal();}
}