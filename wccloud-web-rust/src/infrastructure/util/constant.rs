//! author wcz
use lazy_static::lazy_static;
use xor_str::xor;
use xor_str::encode;
use xor_str::decode;

lazy_static!{
pub static ref   REDIS_CACHE: String = "REDIS_CACHE:".parse().unwrap();
pub static ref   MINIO_ACCESS_KEY: String = xor!("1");
pub static ref   MINIO_SECRET_KEY: String = xor!("1");
pub static ref   MYSQL_URL: String = xor!("mysql://root:9eD4PYotLh9f7u2KJnqxUCyX3kQZSXhn@home0122:3306/wccloud_blog");
pub static ref   REDIS_URL: String = xor!("redis://:V9fqvTFmQuwhWao2AEqzEHDFH9RnA5YZ@home0122:6379/");

}
