use crate::ast::arithmetic::Add;
use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::arithmetic::Minus;
use crate::ast::arithmetic::Numeral;
use crate::ast::arithmetic::Product;
use crate::ast::arithmetic::Uminus;
use crate::ast::arithmetic::Variable;
use crate::ast::boolean::And;
use crate::ast::boolean::Boolean;
use crate::ast::boolean::BooleanExpression;
use crate::ast::boolean::Equal;
use crate::ast::boolean::Great;
use crate::ast::boolean::GreatEqual;
use crate::ast::boolean::Less;
use crate::ast::boolean::LessEqual;
use crate::ast::boolean::Or;
use crate::ast::statement::Skip;
use crate::ast::statement::Statement;
use crate::ast::State;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;

use std::fmt;
use std::fmt::{Display, Error, Formatter};

pub struct TokenVec {
    tokens: Vec<Token>,
}

impl Display for TokenVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for token in &self.tokens {
            // Usa un match per ogni tipo di token
            let token_str = match &token.token_ty {
                TokenType::Number(n) => format!("Number({})", n),
                TokenType::Identifier(id) => format!("Identifier({})", id),
                TokenType::Plus => "Plus".to_string(),
                TokenType::Minus => "Minus".to_string(),
                TokenType::Multiply => "Multiply".to_string(),
                TokenType::Divide => "Divide".to_string(),
                TokenType::Assign => "Assign(:=)".to_string(),
                TokenType::LessEqual => "LessEqual(<=)".to_string(),
                TokenType::Less => "Less(<)".to_string(),
                TokenType::Greater => "Greater(>)".to_string(),
                TokenType::GreatEqual => "GreatEqual(>=)".to_string(),
                TokenType::Equal => "Equal(=)".to_string(),
                TokenType::And => "And(&&)".to_string(),
                TokenType::Or => "Or(||)".to_string(),
                TokenType::Not => "Not(!)".to_string(),
                TokenType::PlusPlus => "PlusPlus(++)".to_string(),
                TokenType::If => "If".to_string(),
                TokenType::Then => "Then".to_string(),
                TokenType::Else => "Else".to_string(),
                TokenType::While => "While".to_string(),
                TokenType::Repeat => "Repeat".to_string(),
                TokenType::Until => "Until".to_string(),
                TokenType::For => "For".to_string(),
                TokenType::Skip => "Skip".to_string(),
                TokenType::True => "True".to_string(),
                TokenType::False => "False".to_string(),
                TokenType::Bra => "Bra (".to_string(),
                TokenType::Ket => "Ket )".to_string(),
                TokenType::CBra => "CBra {".to_string(),
                TokenType::Cket => "Cket }".to_string(),
                TokenType::Semicolon => "Semicolon ; ".to_string(),
            };

            // Scrive il token corrente nel formatter
            writeln!(f, "{}", token_str)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Any {
    BooleanExpression(Box<dyn BooleanExpression>), // Memorizzo i tipi che implementano BooleanExpression
    ArithmeticExpression(Box<dyn ArithmeticExpression>), // Memorizzo i tipi che implementano ArithmeticExpression
    Statement(Box<dyn Statement>), // Memorizzo i tipi che implementano Statement
    Token(Token),                  // Memorizzo i token direttamente
}

impl Any {
    // Funzione per creare un Any da una BooleanExpression
    pub fn from_boolean_expr(expr: Box<dyn BooleanExpression>) -> Self {
        Any::BooleanExpression(expr)
    }

    // Funzione per creare un Any da una ArithmeticExpression
    pub fn from_arithmetic_expr(expr: Box<dyn ArithmeticExpression>) -> Self {
        Any::ArithmeticExpression(expr)
    }

    // Funzione per creare un Any da uno Statement
    pub fn from_statement(stmt: Box<dyn Statement>) -> Self {
        Any::Statement(stmt)
    }

    // Funzione per creare un Any da un Token
    pub fn from_token(token: Token) -> Self {
        Any::Token(token)
    }

    // Funzione per ottenere un riferimento a BooleanExpression (se presente)
    pub fn as_boolean_expr(&self) -> Option<&Box<dyn BooleanExpression>> {
        if let Any::BooleanExpression(expr) = self {
            Some(expr)
        } else {
            None
        }
    }

    // Funzione per ottenere un riferimento a ArithmeticExpression (se presente)
    pub fn as_arithmetic_expr(&self) -> Option<&Box<dyn ArithmeticExpression>> {
        if let Any::ArithmeticExpression(expr) = self {
            Some(expr)
        } else {
            None
        }
    }

    // Funzione per ottenere un riferimento a Statement (se presente)
    pub fn as_statement(&self) -> Option<&Box<dyn Statement>> {
        if let Any::Statement(stmt) = self {
            Some(stmt)
        } else {
            None
        }
    }

    // Funzione per ottenere un riferimento a Token (se presente)
    pub fn as_token(&self) -> Option<&Token> {
        if let Any::Token(token) = self {
            Some(token)
        } else {
            None
        }
    }
}
pub struct AnyVec {
    nodes: Vec<Any>,
}

impl AnyVec {
    pub fn push_boolean_expr(&mut self, expr: Box<dyn BooleanExpression>) {
        self.nodes.push(Any::from_boolean_expr(expr));
    }

    pub fn push_arithmetic_expr(&mut self, expr: Box<dyn ArithmeticExpression>) {
        self.nodes.push(Any::from_arithmetic_expr(expr));
    }

    pub fn push_statement(&mut self, stmt: Box<dyn Statement>) {
        self.nodes.push(Any::from_statement(stmt));
    }

    pub fn push_token(&mut self, token: Token) {
        self.nodes.push(Any::from_token(token));
    }
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

impl Display for AnyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.nodes {
            match node {
                Any::BooleanExpression(expr) => {
                    writeln!(f, "Boolean Expression: {:?}", expr)?;
                }
                Any::ArithmeticExpression(expr) => {
                    writeln!(f, "Arithmetic Expression: {:?}", expr)?;
                }
                Any::Statement(stmt) => {
                    writeln!(f, "Statement: {:?}", stmt)?;
                }
                Any::Token(token) => {
                    writeln!(f, "Token: {:?}", token)?;
                }
            }
        }
        Ok(())
    }
}

pub fn parse_lit(tok_vec: &mut AnyVec, index: &mut usize) {
    if *index >= tok_vec.nodes.len() {
        panic!("parse_lit:: Errore di parsing: indice fuori limite");
    }

    // Controlla che il nodo corrente sia un Token
    match &tok_vec.nodes[*index] {
        Any::Token(token) => match token.token_ty {
            TokenType::Number(value) => {
                // Crea un Numeral e sostituisci il Token con un ArithmeticExpression
                let numeral = Numeral(value);
                let arithmetic_expr = Any::from_arithmetic_expr(Box::new(numeral));

                // Sostituisce il token corrente con l'espressione aritmetica
                tok_vec.nodes[*index] = arithmetic_expr;
            }
            _ => {}
        },
        _ => {
            panic!("parse_lit:: Errore di parsing: il nodo corrente non è un token");
        }
    }
}

pub fn parse_bool_value(tok_vec: &mut AnyVec, index: &mut usize) {
    if *index >= tok_vec.nodes.len() {
        panic!("parse_lit:: Errore di parsing: indice fuori limite");
    }
    match &tok_vec.nodes[*index] {
        Any::Token(token) => match token.token_ty {
            TokenType::True => {
                let booleanv = Boolean(true);

                let bool_expr = Any::from_boolean_expr(Box::new(booleanv));

                // Sostituisce il token corrente con l'espressione booleana
                tok_vec.nodes[*index] = bool_expr;
            }
            TokenType::False => {
                let booleanv = Boolean(false);

                let bool_expr = Any::from_boolean_expr(Box::new(booleanv));

                // Sostituisce il token corrente con l'espressione booleana
                tok_vec.nodes[*index] = bool_expr;
            }
            _ => {}
        },
        _ => {
            panic!("parse_bool_value:: Errore di parsing: il nodo corrente non è un token");
        }
    }
}

pub fn parse_var(tok_vec: &mut AnyVec, index: &mut usize) {
    if *index >= tok_vec.nodes.len() {
        panic!("parse_var:: Errore di parsing: indice fuori limite");
    }

    // Controlla che il nodo corrente sia un Token
    match &tok_vec.nodes[*index] {
        Any::Token(token) => match &token.token_ty {
            TokenType::Identifier(ref id) => {
                // Crea una Variable e sostituisci il Token con un ArithmeticExpression
                let var = Variable {
                    value: id.to_string(),
                };
                let arithmetic_expr = Any::from_arithmetic_expr(Box::new(var));

                // Sostituisce il token corrente con l'espressione aritmetica
                tok_vec.nodes[*index] = arithmetic_expr;
            }
            _ => {}
        },
        _ => {
            panic!("parse_var:: Errore di parsing: il nodo corrente non è un token");
        }
    }
}

pub fn parse_skip(tok_vec: &mut AnyVec, index: &mut usize) {
    if *index >= tok_vec.nodes.len() {
        panic!(" parse_skip:: Errore di parsing: indice fuori limite");
    }

    // Controlla che il nodo corrente sia un Token
    match &tok_vec.nodes[*index] {
        Any::Token(token) => match token.token_ty {
            TokenType::Skip => {
                // Creiamo un'espressione o dichiarazione Skip
                let skip_stmt = Skip;
                let statement_expr = Any::from_statement(Box::new(skip_stmt));

                // Sostituisce il token corrente con la dichiarazione di skip
                tok_vec.nodes[*index] = statement_expr;
            }
            _ => {}
        },
        _ => {
            panic!("parse_skip::  Errore di parsing: il nodo corrente non è un token");
        }
    }
}

pub fn parse_atomic(tok_vec: &mut AnyVec, index: &mut usize) {
    while *index < tok_vec.nodes.len() {
        // Controlla se il nodo attuale è un token
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Number(_) => {
                    parse_lit(tok_vec, index); // Chiama parse_lit per i numeri
                }
                TokenType::Identifier(_) => {
                    parse_var(tok_vec, index); // Chiama parse_var per le variabili
                }
                TokenType::Skip => {
                    parse_skip(tok_vec, index); // Chiama parse_skip per il token Skip
                }
                TokenType::True | TokenType::False => {
                    parse_bool_value(tok_vec, index);
                }
                _ => {}
            }
        }
        // Incrementa l'indice per passare al prossimo token
        *index += 1;
    }
}

pub fn parse_arithmetic_subexpression(tok_vec: &mut AnyVec, index: &mut usize,) -> Box<dyn ArithmeticExpression> {
    // Incrementa l'indice per saltare la parentesi aperta
    *index += 1;

    let start = *index;
    let mut depth = 1; // Traccia la profondità delle parentesi

    // Cerca la parentesi chiusa corrispondente
    while *index < tok_vec.nodes.len() {
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Bra => depth += 1, // Nuova parentesi aperta, aumenta la profondità
                TokenType::Ket => {
                    depth -= 1; // Parentesi chiusa, diminuisci la profondità
                    if depth == 0 {
                        break; // Trovata la parentesi chiusa corrispondente
                    }
                }
                _ => {}
            }
        }
        *index += 1;
    }

    if depth != 0 {
        panic!("Errore di parsing: parentesi chiusa mancante.");
    }
    let num_removed = *index - start;
    // Parsiamo la sottoespressione tra start e index-1
    let mut sub_tok_vec = tok_vec.nodes.drain(start..*index).collect::<Vec<Any>>();

    // Aggiorna l'indice principale in base alla nuova lunghezza di tok_vec
    // Sottrai il numero di elementi drenati (index - start) per correggere l'indice
    *index -= num_removed;

    // Creo il vettore Any contenente solo la sottoespressione da parsare
    let mut sub_any_vec = AnyVec { nodes: sub_tok_vec };

    // Richiama il parsing della sottoespressione
    let mut sub_index = 0; // Indice locale per la sottoespressione
    parse_arithmetic_expression(&mut sub_any_vec, &mut sub_index);

    // Controlla il risultato del parsing
    if let Some(Any::ArithmeticExpression(expr)) = sub_any_vec.nodes.pop() {
        //println!("parsed subexpression {:?}", expr);
        expr // Ritorna l'espressione parsata
    } else {
        panic!("Errore di parsing: expected ArithmeticExpression in sottoespressione.");
    }
}

pub fn parse_bool_subexpression(tok_vec: &mut AnyVec, index: &mut usize,) -> Box<dyn BooleanExpression> {
    // Incrementa l'indice per saltare la parentesi aperta
    *index += 1;

    let start = *index;
    let mut depth = 1; // Traccia la profondità delle parentesi

    // Cerca la parentesi chiusa corrispondente
    while *index < tok_vec.nodes.len() {
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Bra => depth += 1, // Nuova parentesi aperta, aumenta la profondità
                TokenType::Ket => {
                    depth -= 1; // Parentesi chiusa, diminuisci la profondità
                    if depth == 0 {
                        break; // Trovata la parentesi chiusa corrispondente
                    }
                }
                _ => {}
            }
        }
        *index += 1;
    }

    if depth != 0 {
        panic!("Errore di parsing: parentesi chiusa mancante.");
    }
    let num_removed = *index - start;
    // Parsiamo la sottoespressione tra start e index-1
    let mut sub_tok_vec = tok_vec.nodes.drain(start..*index).collect::<Vec<Any>>();

    // Aggiorna l'indice principale in base alla nuova lunghezza di tok_vec
    // Sottrai il numero di elementi drenati (index - start) per correggere l'indice
    *index -= num_removed;

    // Creo il vettore Any contenente solo la sottoespressione da parsare
    let mut sub_any_vec = AnyVec { nodes: sub_tok_vec };

    // Richiama il parsing della sottoespressione
    let mut sub_index = 0; // Indice locale per la sottoespressione
    parse_bool_expression(&mut sub_any_vec, &mut sub_index);

    // Controlla il risultato del parsing
    if let Some(Any::BooleanExpression(expr)) = sub_any_vec.nodes.pop() {
        //println!("parsed subexpression {:?}", expr);
        expr // Ritorna l'espressione parsata
    } else {
        panic!("Errore di parsing: expected ArithmeticExpression in sottoespressione.");
    }
}

pub fn parse_bool_expression(tok_vec: &mut AnyVec, index: &mut usize) {
    println!("index:= {}", index);
    while *index < tok_vec.nodes.len() {
        // Controlla se il nodo attuale è un token
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                //TODO RICERCA UNARY OPERATOR
                TokenType::And => {
                    // Prima dell' and si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!("Errore di parsing: operando sinistro mancante per l'and.");
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::BooleanExpression(expr) => expr,
                        _ => panic!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra dell'and."
                        ),
                    };

                    // Dopo l'and, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per l'and.");
                    }
                    println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_bool_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_bool_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'and."),
                                    }
                                }
                            }
                            Any::BooleanExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'and."),
                                    }
                            }
                            _ => panic!(
                                "Errore di parsing: nodo non riconosciuto a destra dell'and."
                            ),
                        }
                    } else {
                        panic!("Errore di parsing: nessun token trovato a destra dell'and.");
                    };

                    //println!("printing the token vector after all");
                    //let mut j = 0;
                    // while j < tok_vec.nodes.len()
                    // {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j=j+1;
                    // }
                    // Crea l'oggetto And con left e right
                    let and_expr = And { left, right };

                    // Reinserisci l'oggetto And nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(and_expr)));

                    //elimino il token contenente l'operatore and
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Or => {
                    // Prima del or si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!("Errore di parsing: operando sinistro mancante per l'or.");
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::BooleanExpression(expr) => expr,
                        _ => panic!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra dell'or."
                        ),
                    };

                    // Dopo l'or, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per l'or.");
                    }
                    println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_boolean_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_bool_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'or."),
                                    }
                                }
                            }
                            Any::BooleanExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'or."),
                                    }
                            }
                            _ => {
                                panic!("Errore di parsing: nodo non riconosciuto a destra dell'or.")
                            }
                        }
                    } else {
                        panic!("Errore di parsing: nessun token trovato a destra dell'or.");
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len()
                    // {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j=j+1;
                    // }
                    // Crea l'oggetto Add con left e right
                    let or_expr = Or { left, right };

                    // Reinserisci l'oggetto Add nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(or_expr)));

                    //elimino il token contenente l'operatore +
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Equal => {
                    // Prima del = si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano =."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => panic!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano =."),
                    };

                    // Dopo l'=, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per l'op booleano =.");
                    }
                    println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_boolean_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano =."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano =."),
                                    }
                                },
                                _ => panic!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano =."),
                        }
                    } else {
                        panic!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano =."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto Equal con left e right
                    let eq_expr = Equal { left, right };

                    // Reinserisci l'oggetto Equal nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(eq_expr)));

                    //elimino il token contenente l'operatore =
                    tok_vec.nodes.remove(*index);
                }
                TokenType::LessEqual => {
                    // Prima del <= si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano <=."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => panic!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano <=."),
                    };

                    // Dopo il <=, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per l'op booleano <=.");
                    }
                    println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_boolean_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano <=."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano <=."),
                                    }
                                },
                                _ => panic!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano <=."),
                        }
                    } else {
                        panic!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano <=."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto LessEqual con left e right
                    let leq_expr = LessEqual { left, right };

                    // Reinserisci l'oggetto LessEqual nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(leq_expr)));

                    //elimino il token contenente l'operatore <=
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Less => {
                    // Prima del < si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano <."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => panic!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano <."),
                    };

                    // Dopo il <, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per l'op booleano <.");
                    }
                    println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_boolean_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano <."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano <."),
                                    }
                                },
                                _ => panic!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano <."),
                        }
                    } else {
                        panic!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano <."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto Less con left e right
                    let less_expr = Less { left, right };

                    // Reinserisci l'oggetto Less nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(less_expr)));

                    //elimino il token contenente l'operatore <
                    tok_vec.nodes.remove(*index);
                }
                TokenType::GreatEqual => {
                    // Prima del >= si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano >=."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => panic!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano >=."),
                    };

                    // Dopo il >=, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per l'op booleano >=.");
                    }
                    println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_arithmetic_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano >=."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano >=."),
                                    }
                                },
                                _ => panic!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano >=."),
                        }
                    } else {
                        panic!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano >=."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto GreatEqual con left e right
                    let geq_expr = GreatEqual { left, right };

                    // Reinserisci l'oggetto Less nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(geq_expr)));

                    //elimino il token contenente l'operatore <
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Greater => {
                    // Prima del > si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano >."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => panic!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano >."),
                    };

                    // Dopo il >, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per l'op booleano >.");
                    }
                    println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_arithmetic_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano >."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano >."),
                                    }
                                },
                                _ => panic!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano >."),
                        }
                    } else {
                        panic!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano >."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto Grea con left e right
                    let great_expr = Great { left, right };

                    // Reinserisci l'oggetto Less nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(great_expr)));

                    //elimino il token contenente l'operatore <
                    tok_vec.nodes.remove(*index);
                }
                _ => {}
            }
        }
        // Incrementa l'indice per passare al prossimo token
        *index += 1;
    }
}

pub fn parse_arithmetic_unop (tok_vec: &mut AnyVec, index: &mut usize){
    while *index < tok_vec.nodes.len(){
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index){
            match token.token_ty {
                TokenType::Minus => {
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per la negazione unaria.");
                    }
                    // TODO assicurarmi che a sx non ci sia operando sinistro
                    // parsare ciò che ho a dx 
                    *index+=1;
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra del '-' unario."),
                                    }
                                }
                            }
                            Any::ArithmeticExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra del '-' unario."),
                                    }
                            }
                            _ => {
                                panic!("Errore di parsing: nodo non riconosciuto a destra del '-' unario.")
                            }
                        }
                    }else {
                        panic!("Errore di parsing: nessun token trovato a destra del '-' unario.");
                    };

                    let min_expr = Uminus {right};
                    tok_vec.nodes.insert(*index-1, Any::ArithmeticExpression(Box::new(min_expr)));

                }
                _ => { }

            }
        }
    }
}



pub fn parse_arithmetic_expression(tok_vec: &mut AnyVec, index: &mut usize) {
    // RICERCA UNARY OPERATOR
    parse_arithmetic_unop(tok_vec , index);
    //println!("index:= {}", index);
    while *index < tok_vec.nodes.len() {
        // Controlla se il nodo attuale è un token
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Plus => {
                    // Prima del `+` si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!("Errore di parsing: operando sinistro mancante per l'addizione.");
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => panic!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra del '+'."
                        ),
                    };

                    // Dopo il `+`, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per l'addizione.");
                    }
                    println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra del '+'."),
                                    }
                                }
                            }
                            Any::ArithmeticExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra del '+'."),
                                    }
                            }
                            _ => {
                                panic!("Errore di parsing: nodo non riconosciuto a destra del '+'.")
                            }
                        }
                    } else {
                        panic!("Errore di parsing: nessun token trovato a destra del '+'.");
                    };

                    // println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto Add con left e right
                    let add_expr = Add { left, right };

                    // Reinserisci l'oggetto Add nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::ArithmeticExpression(Box::new(add_expr)));

                    //elimino il token contenente l'operatore +
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Multiply => {
                    // Prima del `*` si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!(
                            "Errore di parsing: operando sinistro mancante per la moltiplicazione."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    println!("left operand {:?}", left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => panic!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra del '*'."
                        ),
                    };

                    //println!("printing the token vector after the left elimination");
                    //let mut j = 0;
                    //while j < tok_vec.nodes.len()
                    //{
                    //  println!("{:?}", tok_vec.nodes[j]);
                    //   j=j+1;
                    //}

                    // Dopo il `*`, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!(
                            "Errore di parsing: operando destro mancante per la moltiplicazione."
                        );
                    }
                    //println!("second print index:= {}" , index);

                    // Se trovi una parentesi aperta, esegui parse_subexpression

                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra del '*'."),
                                    }
                                }
                            }
                            //caso in cui ho già un ArithmeticExpression a dx
                            Any::ArithmeticExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra del '*'."),
                                    }
                            }
                            _ => {
                                panic!("Errore di parsing: nodo non riconosciuto a destra del '*'.")
                            }
                        }
                    } else {
                        panic!("Errore di parsing: nessun token trovato a destra del '*'.");
                    };

                    // Crea l'oggetto Product con left e right
                    let prod_expr = Product { left, right };

                    // Reinserisci l'oggetto Add nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::ArithmeticExpression(Box::new(prod_expr)));

                    //elimino il token contenente l'operatore *
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Minus => {
                    // Prima del `-` si trova l'operando sinistro (left)
                    if *index == 0 {
                        panic!("Errore di parsing: operando sinistro mancante per la sottrazione.");
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => panic!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra del '-'."
                        ),
                    };

                    //println!("printing the token vector after the left elimination");
                    //let mut j = 0;
                    //while j < tok_vec.nodes.len()
                    //{
                    //  println!("{:?}", tok_vec.nodes[j]);
                    //   j=j+1;
                    //}

                    // Dopo il `-`, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        panic!("Errore di parsing: operando destro mancante per la sottrazione.");
                    }
                    //println!("second print index:= {}" , index);

                    // Se trovi una parentesi aperta, esegui parse_subexpression

                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra del '-'."),
                                    }
                                }
                            }
                            //caso in cui ho già un ArithmeticExpression a dx
                            Any::ArithmeticExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => panic!("Errore di parsing: attesa espressione aritmetica a destra del '-'."),
                                    }
                            }
                            _ => {
                                panic!("Errore di parsing: nodo non riconosciuto a destra del '-'.")
                            }
                        }
                    } else {
                        panic!("Errore di parsing: nessun token trovato a destra del '-'.");
                    };

                    // Crea l'oggetto Product con left e right
                    let diff_expr = Minus { left, right };

                    // Reinserisci l'oggetto Add nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::ArithmeticExpression(Box::new(diff_expr)));

                    //elimino il token contenente l'operatore *
                    tok_vec.nodes.remove(*index);
                }

                _ => {}
            }
        }
        // Incrementa l'indice per passare al prossimo token
        *index += 1;
    }
}


//TODO GENERAL: IMPLEMENTA RICERCA PER UNARY OPERATORS, POI PARSE STATEMENT, POI EVALUATION DELL'AST
pub fn parse(program: String, initial_state: String) {
    //cleaning the input from whitespaces
    let cleanp = program.trim();

    //let the lexer work (string->AnyVec)
    let tokens = Lexer::tokenize(cleanp.to_owned());
    let tokenized_program = TokenVec { tokens };
    let state_tokens = Lexer::tokenize(initial_state);
    let tokenized_state = TokenVec {
        tokens: state_tokens,
    };
    //print!("tokenized initial state: {}", parsed_state);
    //print!("tokenized program code: {}", pre_ast);

    //let's build the ast! (AnyVec->Statement)
    let mut any_vec = AnyVec::new();
    for token in tokenized_program.tokens {
        any_vec.push_token(token);
    }

    let mut index = 0 as usize;
    // literals
    parse_atomic(&mut any_vec, &mut index);
    index = 0;

    println!(" atomic terms parsed");
    //let mut j = 0;
    //while j < any_vec.nodes.len()
    //{
    //    println!("{:?}", any_vec.nodes[j]);
    //    j=j+1;
    //}

    //arithmetic expressions
    parse_arithmetic_expression(&mut any_vec, &mut index);
    index = 0;
    //boolean expressions
    parse_bool_expression(&mut any_vec, &mut index);
    index = 0;
    //statements

    println!(" expressions parsed");
    let mut i = 0;
    while i < any_vec.nodes.len() {
        println!("{:?}", any_vec.nodes[i]);
        i = i + 1;
    }

    //statements

    // evaluate the final statement
    //occhio al caso angeli degli spazi cancellati: 10- -10
}
