#[macro_export]
macro_rules! render {
    {
        @init($chan_setter: expr, $cache_box: expr, $content: expr);
        $($tt:tt)*
    } => {
        $crate::structure::Node::finish(
            $crate::build!{
                @init($chan_setter);
                $($tt)*
            },
            $cache_box,
            $content
        )
    };
}

#[macro_export]
macro_rules! render_fn {
    {
        @init($slf:ident);
        $($tt:tt)*
    } => {
        fn render<'r>(
            $slf: &mut Self,
            _: Self::Props<'_>,
            _: &impl style::StyleContainer,
            __region: $crate::primary::Region,
            __cache_box_for_children: &mut $crate::CacheBox,
            __event_dispatcher: &$crate::event::EventDispatcher,
            _: $crate::structure::Slot<
                impl $crate::structure::StructureBuilder +
                $crate::structure::VisitIter<Self::ChildProps<'r>>
            >,
            mut __content: $crate::element::RenderContent,
        ) -> $crate::Result<()> {
            $crate::structure::StructureBuilder::into_rendering(
                $crate::build! {
                    @init(__event_dispatcher);
                    $($tt)*
                },
                __cache_box_for_children,
                __content.inherit(),
            ).finish(__region)
        }
    };
}
