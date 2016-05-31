//! WARNING: This file is generated, derived from table public.blog, DO NOT EDIT

use gen::column;
use gen::schema;
use gen::table;
use serde_json::Value as Json;
use serde_json::value::ToJson;
use sporm::dao::Dao;
use sporm::dao::IsDao;
use sporm::dao::Type;
use sporm::dao::Value;
use sporm::query::Operand;
use sporm::table::Column;
use sporm::table::IsTable;
use sporm::table::Table;



#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Blog {
    /// primary
    /// default: 'nextval('blog_id_seq'::regclass)'
    /// not nullable 
    /// db data type: bigint
    pub id: i64,
    /// db data type: character varying
    pub content: Option<String>,
    /// db data type: character varying
    pub created_time: Option<String>,
    /// db data type: character varying
    pub title: Option<String>,

}



impl IsDao for Blog {
    fn from_dao(dao: &Dao) -> Self {
        Blog {
            id: dao.get(column::id),
            title: dao.get_opt(column::title),
            content: dao.get_opt(column::content),
            created_time: dao.get_opt(column::created_time),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set(column::id, &self.id);
        match self.title {
            Some(ref _value) => dao.set(column::title, _value),
            None => dao.set_null(column::title)
        }
        match self.content {
            Some(ref _value) => dao.set(column::content, _value),
            None => dao.set_null(column::content)
        }
        match self.created_time {
            Some(ref _value) => dao.set(column::created_time, _value),
            None => dao.set_null(column::created_time)
        }
        dao
    }
}

impl ToJson for Blog {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Blog {

    fn default() -> Self {
        Blog{
            id: Default::default(),
            title: Default::default(),
            content: Default::default(),
            created_time: Default::default(),
        }
    }
}

impl IsTable for Blog {

    fn table() -> Table {
        Table {
            schema: Some(schema::public.to_owned()),
            name: table::blog.to_owned(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![
                Column {
                    name: column::id.to_owned(),
                    data_type: Type::I64,
                    db_data_type: "bigint".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some(Operand::Value(Value::String("'nextval('blog_id_seq'::regclass)'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::title.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::content.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::created_time.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
            ],
            is_view: false,
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static id: &'static str = "blog.id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static title: &'static str = "blog.title";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static content: &'static str = "blog.content";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_time: &'static str = "blog.created_time";
