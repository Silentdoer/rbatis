use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct UpdateNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}


impl SqlNode for UpdateNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        unimplemented!()
    }

    fn print(&self) -> String {
        let mut result="\n<update ".to_string();
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        result=result+" \n</update>";
        return result;
    }
}