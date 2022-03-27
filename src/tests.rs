//     Copyright (C) 2022  cleonyc
// Licensed under GNU Affero General Public License. https://www.gnu.org/licenses/agpl-3.0.en.html
use crate::school_code::get_school_code;

#[tokio::test]
async fn test() {
    assert_eq!(get_school_code("Brooklyn Tech").await.unwrap(), "K430");
    assert_eq!(get_school_code("K430").await.unwrap(), "K430");
}
