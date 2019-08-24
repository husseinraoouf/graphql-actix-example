use super::schema::cats;

#[derive(Queryable)]
pub struct Cat {
    pub id: i32,
    pub name: String,
}


#[derive(Insertable)]
#[table_name="cats"]
pub struct NewCat {
    pub name: String,
}