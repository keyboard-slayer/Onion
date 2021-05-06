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


#[derive(Clone, Debug, PartialOrd)]
pub enum OnionRet
{
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<OnionRet>),
    Symbol(String),
    Fn(fn(Vec<OnionRet>) -> OnionRet),
}

impl OnionRet 
{
    pub fn append(&mut self, token: OnionRet)
    {
        if let OnionRet::List(arr) = self 
        {
            arr.push(token)
        }
    }

    pub fn type_name(self) -> String 
    {
        match self 
        {
            OnionRet::Nil => String::from("nil"),
            OnionRet::Bool(_) => String::from("bool"),
            OnionRet::Int(_) => String::from("int"),
            OnionRet::Str(_) => String::from("str"),
            OnionRet::List(_) => String::from("list"),
            OnionRet::Symbol(_) => String::from("symbol"),
            OnionRet::Float(_) => String::from("float"),
            OnionRet::Fn(_) => String::from("function")
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

impl PartialEq for OnionRet 
{
    fn eq(&self, other: &Self) -> bool 
    {
        if self.clone().type_name() != other.clone().type_name()
        {
            return false;
        }

        match self 
        {
            OnionRet::Nil => if let OnionRet::Nil = other { return true; }
            OnionRet::Bool(b) => if let OnionRet::Bool(b2) = other { return b == b2; }
            OnionRet::Int(i) => if let OnionRet::Int(i2) = other {return i == i2; }
            OnionRet::Str(s) => if let OnionRet::Str(s2) = other {return s == s2; }
            OnionRet::List(v) => if let OnionRet::List(v2) = other {return v == v2; }
            OnionRet::Float(f) => if let OnionRet::Float(f2) = other {return f == f2; }
            OnionRet::Symbol(_) => panic!(),
            OnionRet::Fn(_) => panic!(),
        }

        false
    }
}

/*
impl PartialOrd for OnionRet 
{
    fn gt(&self, other: &Self) -> bool 
    {
        if type_of(self) != type_of(other)
        {
            return false;
        }

        match self 
        {
            OnionRet::Nil => if let OnionRet::Nil = other { return true; }
            OnionRet::Bool(b) => if let OnionRet::Bool(b2) = other { return b > b2; }
            OnionRet::Int(i) => if let OnionRet::Int(i2) = other {return i > i2; }
            OnionRet::Str(s) => if let OnionRet::Str(s2) = other {return s > s2; }
            OnionRet::List(v) => if let OnionRet::List(v2) = other {return v > v2; }
            OnionRet::Symbol(_) => panic!(),
            OnionRet::Fn(_) => panic!(),
        }

        false
    }
}*/