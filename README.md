## A crate built to access google firebase's cloud firestore database using its REST api.

### This crate is very much under construction and should not be used at this time.
(also unofficial and very basic)
To-do list:
- [x] Initial setup of directory structure
- [x] `Collection`, `Document`, and `Firestore` struct basic parent - child hierarchy
- [ ] Custom error type(s)
- [x] Raw string GET request from firestore
- [x] parse the raw string GET request into a `serde_json::Value` object
- [ ] Set up github CI/CD pipeline
- [ ] Authentication