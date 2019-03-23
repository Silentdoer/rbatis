use std::collections::HashMap;
use core::borrow::Borrow;
use crate::lib::RustExpressionEngine::node::Node;
use crate::lib::RustExpressionEngine::node::NodeType::{NOpt, NBinary};
use crate::lib::RustExpressionEngine::runtime::{IsNumber, OptMap, ParserTokens};
use std::collections::linked_list::LinkedList;


//TODO 解决bug

pub fn Parser(express: String, optMap: &OptMap) -> (Node, String) {
    let tokens = ParserTokens(&express);

    let mut nodes = vec![];
    for item in tokens {
        let node = Node::parser(item);
//        if err != "" {
//            return (Node::New(String::new()), err);
//        }
        nodes.push(node);
    }
//    //TODO check nodes
//
//
    for item in optMap.priorityArray() {
        findReplaceOpt(optMap, &express, &item, &mut nodes);
    }
//
    for item in &nodes {
//        if item.Type()==NBinary{
//            //println!("{}",item.Data)
//        }
    }
    return (nodes[0].clone(), String::new());
}

fn findReplaceOpt(optMap: &OptMap, express: &String, operator: &str, nodeArg: &mut Vec<Node>) {

    //let nodes=vec![];
    let mut index = 0 as i32;
    let nodeArgLen = nodeArg.len();
    for item in nodeArg.clone() {
        let itemType = item.nodeType();
        if itemType as i32 == NOpt as i32 && operator == item.opt().unwrap() {
            let leftIndex = (index - 1) as usize;
            let rightIndex = (index + 1) as usize;
            let left = nodeArg[leftIndex].clone();
            let right = nodeArg[rightIndex].clone();
            let binaryNode = Node::newBinary(left, right, item.opt().unwrap());
            println!("binNode={}", &binaryNode.nodeType());

            nodeArg.remove(rightIndex);
            nodeArg.remove(index as usize);
            nodeArg.remove(leftIndex);

            nodeArg.insert(leftIndex, binaryNode);
            if haveOpt(nodeArg) {
                findReplaceOpt(optMap, express, operator, nodeArg);
            }
            break;
        }
        index = index + 1;
    }
}

fn haveOpt(nodeArg: &mut Vec<Node>) -> bool {
    for item in nodeArg {
        if item.nodeType() as i32 == NOpt as i32 {
            return true;
        }
    }
    return false;
}