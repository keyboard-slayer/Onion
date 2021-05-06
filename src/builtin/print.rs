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

use crate::onionret::OnionRet;

pub fn println(args: Vec<OnionRet>) -> OnionRet
{
    let mut to_print: Vec<String> = vec![];

    for arg in args 
    {
        match arg 
        {
            OnionRet::Nil => to_print.push(String::from("Nil")),
            OnionRet::Bool(b) => {
                if b 
                {
                    to_print.push(String::from("#t"));
                }
                else 
                {
                    to_print.push(String::from("#f"));
                }
            }

            OnionRet::Str(s) => to_print.push(s),
            OnionRet::List(_) => eprintln!("TODO !"), /* Make a format function */
            OnionRet::Int(d) => to_print.push(d.to_string()),
            OnionRet::Symbol(_) => eprintln!("TODO !"),
            OnionRet::Fn(_) => panic!(),
        }
    }

    println!("{}", to_print.join(" "));
    OnionRet::Nil
}