pub struct PaginatedVec<'a, T: 'a> {
    per_page: usize,
    pages: Vec<Vec<&'a T>>,
}

impl<'a, T> PaginatedVec<'a, T> {
    pub fn from_vec(vec: &'a [T], per_page: usize) -> PaginatedVec<T> {
        PaginatedVec {
            per_page,
            pages: vec
                .chunks(per_page)
                .map(|page| page.iter().collect::<Vec<&'a T>>())
                .collect::<Vec<Vec<&'a T>>>(),
        }
    }

    pub fn page(&'a self, index: usize) -> Option<(usize, &Vec<&'a T>)> {
        self.pages
            .get(index)
            .map(|page| (index % self.per_page, page))
    }

    pub fn get_max_pages(&self) -> usize {
        self.pages.len()
    }
}
