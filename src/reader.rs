/*
 * Copyright (C) 2021 Jordan DALCQ (Keyboard-Slayer) & Contributors
 *
 * This file is part of Onion.
 *
 * Onion is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Onion is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Onion.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use regex::Regex;

#[derive(Clone, Debug)]
struct Reader 
{
    tokens: Vec<String>,
    pos: usize
}

#[derive(Clone, Debug)]
pub enum OnionRet
{
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    List(Vec<OnionRet>),
    Symbol(String),
    Fn(fn(OnionRet) -> OnionRet),
}

impl OnionRet 
{
    fn append(&mut self, token: OnionRet)
    {
        if let OnionRet::List(arr) = self 
        {
            arr.push(token)
        }
    }

    pub fn is_nil(self) -> bool
    {
        if let OnionRet::Nil = self 
        {
            true
        }
        else
        {
            false
        }
    }
}

impl Reader 
{
    fn next(&mut self) -> String
    {
        let return_value = self.peek();
        self.pos += 1;

        return_value
    }

    fn previous(&mut self) -> String 
    {
        self.pos -= 1;
        let return_value = self.peek();
        self.pos += 1;

        return_value
    }

    fn peek(&mut self) -> String
    {
        if let Some(token) = self.tokens.get(self.pos)
        {
            String::from(token)
        }
        else 
        {
            String::from("Eof")
        }
    }
}

fn tokenize(content: String) -> Vec<String>
{
    let mut result = vec![];
    let regex = Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###).unwrap();
    
    for cap in regex.captures_iter(&content)
    {
        if cap[1].starts_with(";")
        {
            continue;
        }

        result.push(String::from(&cap[1]));
    }

    result
}

fn read_list(reader: &mut Reader) -> OnionRet
{
    let mut seq: Vec<OnionRet> = vec![];

    loop 
    {
        let token = reader.peek();
        if token == "Eof"
        {
            panic!("Expected )");
        }

        if token == ")"
        {
            break;
        }

        seq.push(read_form(reader));
    }

    reader.next();
    OnionRet::List(seq)
}

fn read_atom(reader: &mut Reader) -> OnionRet 
{
    let int_re = Regex::new(r"^-?[0-9]+$").unwrap();
    let str_re = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();

    let token = reader.next();

    match token.as_str()
    {
        "nil" => OnionRet::Nil,
        "#f" => OnionRet::Bool(false),
        "#t" => OnionRet::Bool(true),
        _ => 
        {
            if int_re.is_match(&token) 
            {
                OnionRet::Int(token.parse().unwrap())
            }
            else if str_re.is_match(&token)
            {
                OnionRet::Str(token[1..token.len()-1].to_string())
            }
            else 
            {
                OnionRet::Symbol(token.to_string())
            }
        }
    }
}

fn read_form(reader: &mut Reader) -> OnionRet
{
    let token = reader.peek();

    match token.as_str()
    {
        "(" => {
            reader.next();
            read_list(reader)
        }

        ")" => {
            println!("CURRENT: {}, PREVIOUS: {}", reader.peek(), reader.previous());
            panic!();
        }
        _ => read_atom(reader)
    }
}

pub fn read_str(content: String) -> OnionRet
{
    let mut return_value: OnionRet = OnionRet::List(vec![]);
    let tokens = tokenize(content);
    let mut reader = Reader {
        tokens: tokens,
        pos: 0
    };


    while reader.peek() != "Eof"
    {
        return_value.append(read_form(&mut reader));
    }
    
    return_value
}