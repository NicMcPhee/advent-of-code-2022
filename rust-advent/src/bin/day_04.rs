use std::fs;

struct SectionAssignment {
    start: u8,
    end:   u8,
}

impl SectionAssignment {
    fn contains(&self, other: &SectionAssignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &SectionAssignment) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
            || (other.start <= self.start && self.start <= other.end)
            || (other.start <= self.end && self.end <= other.end)
    }
}

impl From<&str> for SectionAssignment {
    fn from(s: &str) -> Self {
        let mut parts
            = s.split('-')
               .map(|s| s.parse::<u8>().unwrap());
        SectionAssignment {
            start: parts.next().unwrap(),
            end:   parts.next().unwrap(),
        }
    }
}

struct AssignmentPair {
    left:  SectionAssignment,
    right: SectionAssignment,
}

impl From<&str> for AssignmentPair {
    fn from(s: &str) -> Self {
        let mut parts
            = s.split(",")
               .map(SectionAssignment::from);
        AssignmentPair {
            left:  parts.next().unwrap(),
            right: parts.next().unwrap()
        }
    }
}

impl AssignmentPair {
    fn contains(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn overlaps(&self) -> bool {
        self.left.overlaps(&self.right)
    }
}

fn main() {
    let contents = fs::read_to_string("../inputs/day_04.input")
        .expect("Should have been able to read the file");
    
    let pairs 
        = contents.split('\n')
                  .map(AssignmentPair::from);

    let num_contains
        = pairs.clone()
               .filter(|p| p.contains())
               .count();

    println!("The number of complete containments is {num_contains}.");

    let num_overlaps
        = pairs.filter(|p| p.overlaps())
               .count();
    
    println!("The number of overlaps is {num_overlaps}");
}