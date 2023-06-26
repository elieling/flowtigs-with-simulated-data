#[cfg(test)]
mod tests {
    use std::vec::Vec;
    use crate::safe_paths::safe_paths;

    #[test]
    fn simple_graph() {
        let safe_paths = safe_paths("data/test_data/longer_k4.edgelist", 4);
        let mut result = Vec::new();
        for element in safe_paths {
            result.push(element);
        }
        result.sort();


        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "ACGCCCGTTTTTTACG");
        assert_eq!(result[1], "ACGGGCGTAAAAAACG");
        assert_eq!(result[2], "ACGT");
        assert_eq!(result[3], "CGTACG");
    }



    #[test]
    fn right_outflow() {
        let safe_paths = safe_paths("data/test_data/outflow_k2.edgelist", 2);
        let mut result = Vec::new();
        for element in safe_paths {
            result.push(element);
        }
        result.sort();


        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "AA");
        assert_eq!(result[1], "ACG");
        assert_eq!(result[2], "CC");
        assert_eq!(result[3], "GAC");
    }



    #[test]
    fn two_cycles() {
        let safe_paths = safe_paths("data/test_data/two_cycles_k5.edgelist", 5);
        let mut result = Vec::new();
        for element in safe_paths {
            result.push(element);
        }
        result.sort();


        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "AAAAAAAAAA");
        assert_eq!(result[1], "AAAACGTAAAA");
        assert_eq!(result[2], "AACGTAAAACG");
        assert_eq!(result[3], "ACGTAAAACGT");
        assert_eq!(result[4], "GTAAAACGTAA");
    }



    #[test]
    fn only_maximals() {
        let safe_paths = safe_paths("data/test_data/maximal_k3.edgelist", 3);
        let mut result = Vec::new();
        for element in safe_paths {
            result.push(element);
        }
        result.sort();


        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "ACG");
        assert_eq!(result[1], "ACTCGTAC");
        assert_eq!(result[2], "CGAC");
        assert_eq!(result[3], "CGAGTACG");
    }



    #[test]
    fn should_fail() {
        let _ = safe_paths("data/fake.edgelist", 10);
        assert_eq!(1, 1);
    }
}