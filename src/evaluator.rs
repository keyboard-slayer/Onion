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

use crate::builtin::get_builtin;
use crate::onionret::OnionRet;
use std::process::exit;


fn execute(lst: Vec<OnionRet>) -> OnionRet 
{
    if let OnionRet::Fn(func) = lst.get(0).unwrap()
    {
        let mut arg = lst.clone();
        arg.remove(0);
        return func(arg);
    }
    else 
    {
        let arg = lst.get(0).unwrap().clone();

        if !arg.is_nil() 
        {
            eprintln!("Expected a function");
            exit(1);
        }
        else 
        {
            return OnionRet::Nil;
        }
    }
}

pub fn eval(tokens: OnionRet) -> OnionRet
{
    let funcs = get_builtin();

    let mut _return_value: OnionRet = OnionRet::Nil;

    match tokens 
    {
        OnionRet::List(l) => {
            let mut lst: Vec<OnionRet> = vec![];
            for tok in l.iter() 
            {
                lst.push(eval(tok.clone()));
            }
            
            _return_value = execute(lst);
        }
        
        OnionRet::Symbol(s) => {
            if let Some(func) = funcs.get(&s) 
            {
                _return_value = func.clone();
            }
            else 
            {
                eprintln!("{} undefined", s);
                exit(1);
            }
        }

        _ => _return_value = tokens,
    }

    _return_value
}