use re_arrow_store::{DataStore, LatestAtQuery, RangeQuery, TimeInt, TimeRange, Timeline};
use re_data_store::ExtraQueryHistory;
use re_log_types::{EntityPath, LegacyComponent};
use re_types::{Archetype, ComponentName};

use crate::{
    query_archetype, query_entity_with_primary, range::range_archetype, range_entity_with_primary,
    ArchetypeView, EntityView,
};

/// Either dispatch to `query_entity_with_primary` or `range_entity_with_primary`
/// depending on whether `ExtraQueryHistory` is set.
pub fn query_primary_with_history<
    'a,
    Primary: LegacyComponent + re_types::Component + 'a,
    const N: usize,
>(
    store: &'a DataStore,
    timeline: &'a Timeline,
    time: &'a TimeInt,
    history: &ExtraQueryHistory,
    ent_path: &'a EntityPath,
    components: [ComponentName; N],
) -> crate::Result<impl Iterator<Item = EntityView<Primary>> + 'a> {
    let visible_history = match timeline.typ() {
        re_log_types::TimeType::Time => history.nanos,
        re_log_types::TimeType::Sequence => history.sequences,
    };

    if visible_history == 0 {
        let latest_query = LatestAtQuery::new(*timeline, *time);
        let latest =
            query_entity_with_primary::<Primary>(store, &latest_query, ent_path, &components)?;

        Ok(itertools::Either::Left(std::iter::once(latest)))
    } else {
        let min_time = *time - TimeInt::from(visible_history);
        let range_query = RangeQuery::new(*timeline, TimeRange::new(min_time, *time));

        let range =
            range_entity_with_primary::<Primary, N>(store, &range_query, ent_path, components);

        Ok(itertools::Either::Right(range.map(|(_, entity)| entity)))
    }
}

pub fn query_archetype_with_history<'a, A: Archetype + 'a, const N: usize>(
    store: &'a DataStore,
    timeline: &'a Timeline,
    time: &'a TimeInt,
    history: &ExtraQueryHistory,
    ent_path: &'a EntityPath,
) -> crate::Result<impl Iterator<Item = ArchetypeView<A>> + 'a> {
    let visible_history = match timeline.typ() {
        re_log_types::TimeType::Time => history.nanos,
        re_log_types::TimeType::Sequence => history.sequences,
    };

    if visible_history == 0 {
        let latest_query = LatestAtQuery::new(*timeline, *time);
        let latest = query_archetype::<A>(store, &latest_query, ent_path)?;

        Ok(itertools::Either::Left(std::iter::once(latest)))
    } else {
        let min_time = *time - TimeInt::from(visible_history);
        let range_query = RangeQuery::new(*timeline, TimeRange::new(min_time, *time));

        let range = range_archetype::<A, N>(store, &range_query, ent_path);

        Ok(itertools::Either::Right(range.map(|(_, entity)| entity)))
    }
}
