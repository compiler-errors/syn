// Copyright 2018 Syn Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "fold")]
pub mod fold {
    use punctuated::{Element, Punctuated};
    use fold::Fold;
    use proc_macro2::Span;

    pub trait FoldHelper {
        type Item;
        fn lift<F>(self, f: F) -> Self
        where
            F: FnMut(Self::Item) -> Self::Item;
    }

    impl<T> FoldHelper for Vec<T> {
        type Item = T;
        fn lift<F>(self, f: F) -> Self
        where
            F: FnMut(Self::Item) -> Self::Item,
        {
            self.into_iter().map(f).collect()
        }
    }

    impl<T, U> FoldHelper for Punctuated<T, U> {
        type Item = T;
        fn lift<F>(self, mut f: F) -> Self
        where
            F: FnMut(Self::Item) -> Self::Item,
        {
            self.into_elements()
                .map(Element::into_tuple)
                .map(|(t, u)| Element::new(f(t), u))
                .collect()
        }
    }

    pub fn tokens_helper<F: Fold + ?Sized, S: Spans>(folder: &mut F, spans: &S) -> S {
        spans.fold(folder)
    }

    pub trait Spans {
        fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self;
    }

    impl Spans for Span {
        fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self {
            folder.fold_span(*self)
        }
    }

    impl Spans for [Span; 1] {
        fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self {
            [folder.fold_span(self[0])]
        }
    }

    impl Spans for [Span; 2] {
        fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self {
            [folder.fold_span(self[0]), folder.fold_span(self[1])]
        }
    }

    impl Spans for [Span; 3] {
        fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self {
            [
                folder.fold_span(self[0]),
                folder.fold_span(self[1]),
                folder.fold_span(self[2]),
            ]
        }
    }
}

#[cfg(feature = "visit")]
pub mod visit {
    use proc_macro2::Span;
    use visit::Visit;

    pub fn tokens_helper<'ast, V: Visit<'ast> + ?Sized, S: Spans>(
        visitor: &mut V,
        spans: &'ast S,
    ) {
        spans.visit(visitor);
    }

    pub trait Spans {
        fn visit<'ast, V: Visit<'ast> + ?Sized>(&'ast self, visitor: &mut V);
    }

    impl Spans for Span {
        fn visit<'ast, V: Visit<'ast> + ?Sized>(&'ast self, visitor: &mut V) {
            visitor.visit_span(self);
        }
    }

    impl Spans for [Span; 1] {
        fn visit<'ast, V: Visit<'ast> + ?Sized>(&'ast self, visitor: &mut V) {
            visitor.visit_span(&self[0]);
        }
    }

    impl Spans for [Span; 2] {
        fn visit<'ast, V: Visit<'ast> + ?Sized>(&'ast self, visitor: &mut V) {
            visitor.visit_span(&self[0]);
            visitor.visit_span(&self[1]);
        }
    }

    impl Spans for [Span; 3] {
        fn visit<'ast, V: Visit<'ast> + ?Sized>(&'ast self, visitor: &mut V) {
            visitor.visit_span(&self[0]);
            visitor.visit_span(&self[1]);
            visitor.visit_span(&self[2]);
        }
    }
}

#[cfg(feature = "visit_mut")]
pub mod visit_mut {
    use proc_macro2::Span;
    use visit_mut::VisitMut;

    pub fn tokens_helper<V: VisitMut + ?Sized, S: Spans>(visitor: &mut V, spans: &mut S) {
        spans.visit_mut(visitor);
    }

    pub trait Spans {
        fn visit_mut<V: VisitMut + ?Sized>(&mut self, visitor: &mut V);
    }

    impl Spans for Span {
        fn visit_mut<V: VisitMut + ?Sized>(&mut self, visitor: &mut V) {
            visitor.visit_span_mut(self);
        }
    }

    impl Spans for [Span; 1] {
        fn visit_mut<V: VisitMut + ?Sized>(&mut self, visitor: &mut V) {
            visitor.visit_span_mut(&mut self[0]);
        }
    }

    impl Spans for [Span; 2] {
        fn visit_mut<V: VisitMut + ?Sized>(&mut self, visitor: &mut V) {
            visitor.visit_span_mut(&mut self[0]);
            visitor.visit_span_mut(&mut self[1]);
        }
    }

    impl Spans for [Span; 3] {
        fn visit_mut<V: VisitMut + ?Sized>(&mut self, visitor: &mut V) {
            visitor.visit_span_mut(&mut self[0]);
            visitor.visit_span_mut(&mut self[1]);
            visitor.visit_span_mut(&mut self[2]);
        }
    }
}
