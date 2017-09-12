#[macro_use]
extern crate nom;
extern crate av_format;
extern crate av_data;

#[macro_use]
pub mod permutation;
#[macro_use]
pub mod ebml;
pub mod elements;
pub mod demuxer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
