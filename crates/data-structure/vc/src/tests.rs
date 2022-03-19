use common::*;

use crate::*;

#[test]
fn test_internal() {
    let mut internal = VersionControlInternal::<i32, String>::new();
    internal.update(0, Arc::new("0".into()), &[1]);
    should_eq!(internal.uid(0).raw, 0);
    internal.update(1, Arc::new("1".into()), &[]);
    should_eq!(internal.uid(0).raw, 0);
    should_eq!(internal.uid(1).raw, 1);
    internal.update(1, Arc::new("1_".into()), &[]);
    should_eq!(internal.uid(0).raw, 3);
    should_eq!(internal.uid(1).raw, 2);
    internal.update(1, Arc::new("1_".into()), &[0]);
    should_eq!(internal.uid(0).raw, 3);
    should_eq!(internal.uid(1).raw, 2);
    internal.update(0, Arc::new("0_".into()), &[]);
    should_eq!(internal.uid(0).raw, 4);
    should_eq!(internal.uid(1).raw, 5);
    internal.update(1, Arc::new("1__".into()), &[0]);
    should_eq!(internal.uid(0).raw, 4);
    should_eq!(internal.uid(1).raw, 6);
}
