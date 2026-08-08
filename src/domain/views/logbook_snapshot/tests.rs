use crate::domain::*;
use crate::test_support::Fixture;

// ------------------------------------
// Helpers
fn snapshot(fixture: &Fixture, as_of: Date) -> LogbookSnapshot {
    LogbookSnapshot::from(&fixture.logbook, as_of)
}

fn medication_snapshot<'a>(s: &'a LogbookSnapshot, name: &str) -> &'a MedicationSnapshot {
    s.medications()
        .iter()
        .find(|m| m.medication().name() == name)
        .unwrap_or_else(|| panic!("medication snapshot `{name}` missing"))
}

fn script_snapshot<'a>(s: &'a LogbookSnapshot, script_id: ScriptId) -> &'a ScriptSnapshot {
    s.scripts()
        .iter()
        .find(|s| s.script().id() == script_id)
        .unwrap_or_else(|| panic!("script snapshot missing"))
}

fn item_snapshot<'a>(script: &'a ScriptSnapshot, med_name: &str) -> &'a ScriptItemSnapshot {
    script
        .items()
        .iter()
        .find(|i| i.medication().name() == med_name)
        .unwrap_or_else(|| panic!("script item `{med_name}` missing"))
}

// ------------------------------------
#[test]
fn empty_logbook_produces_empty_snapshot() {
    let fixture = Fixture::new();
    let snapshot = snapshot(&fixture, fixture.today());

    assert_eq!(snapshot.as_of(), fixture.today());
    assert!(snapshot.medications().is_empty());
    assert!(snapshot.scripts().is_empty());
}

#[test]
fn medication_without_any_script_is_critical() {
    let fixture = Fixture::new().medication("med01");
    let snapshot = snapshot(&fixture, fixture.today());

    let medication = medication_snapshot(&snapshot, "med01");
    assert_eq!(medication.health(), Health::Critical);
    assert_eq!(medication.remaining_supplies(), SupplyCount::ZERO);
}

#[test]
fn current_script_with_unused_repeats_is_ok() {
    let fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 2)]);

    let snapshot = snapshot(&fixture, fixture.today());
    let script_id = fixture.script_id("script01");

    let script = script_snapshot(&snapshot, script_id);
    assert_eq!(script.health(), Health::Ok);

    let item = item_snapshot(script, "med01");
    assert_eq!(item.remaining_supplies(), SupplyCount::from(3));
    assert_eq!(item.health(), Health::Ok);

    let medication = medication_snapshot(&snapshot, "med01");
    assert_eq!(medication.health(), Health::Ok);
    assert_eq!(medication.remaining_supplies(), SupplyCount::from(3));
}

#[test]
fn supplies_issued_on_or_before_as_of_reduce_remaining() {
    let fixture = Fixture::new();
    let yesterday = fixture.yesterday();
    let today = fixture.today();

    let fixture = fixture
        .medication("med01")
        .expiring_script("script01", &[("med01", 2)])
        .supply("supply01", "script01", "med01", yesterday)
        .supply("supply02", "script01", "med01", today);

    let snapshot = snapshot(&fixture, today);
    let script_id = fixture.script_id("script01");

    let item = item_snapshot(script_snapshot(&snapshot, script_id), "med01");
    assert_eq!(item.remaining_supplies(), SupplyCount::ONE);
    assert_eq!(item.health(), Health::Attention);

    let medication = medication_snapshot(&snapshot, "med01");
    assert_eq!(medication.remaining_supplies(), SupplyCount::ONE);
    assert_eq!(medication.health(), Health::Attention);
}

#[test]
fn supplies_issued_after_as_of_are_ignored() {
    let fixture = Fixture::new();
    let future = fixture.future();

    let fixture = fixture
        .medication("med01")
        .current_script("script01", &[("med01", 1)])
        .supply("supply01", "script01", "med01", future);

    let snapshot = snapshot(&fixture, fixture.today());
    let script_id = fixture.script_id("script01");

    let item = item_snapshot(script_snapshot(&snapshot, script_id), "med01");
    assert_eq!(item.remaining_supplies(), SupplyCount::from(2));
    assert_eq!(item.health(), Health::Ok);
}

#[test]
fn last_repeat_is_attention() {
    let fixture = Fixture::new();
    let today = fixture.today();

    let fixture = fixture
        .medication("med01")
        .current_script("script01", &[("med01", 1)])
        .supply("supply01", "script01", "med01", today);

    let snapshot = snapshot(&fixture, today);
    let item = item_snapshot(
        script_snapshot(&snapshot, fixture.script_id("script01")),
        "med01",
    );

    assert_eq!(item.remaining_supplies(), SupplyCount::ONE);
    assert_eq!(item.health(), Health::Attention);
}

#[test]
fn no_repeats_left_is_critical() {
    let fixture = Fixture::new();
    let today = fixture.today();

    let fixture = fixture
        .medication("med01")
        .current_script("script01", &[("med01", 0)])
        .supply("supply01", "script01", "med01", today);

    let snapshot = snapshot(&fixture, today);
    let script_id = fixture.script_id("script01");

    let item = item_snapshot(script_snapshot(&snapshot, script_id), "med01");
    assert_eq!(item.remaining_supplies(), SupplyCount::ZERO);
    assert_eq!(item.health(), Health::Critical);

    assert_eq!(
        script_snapshot(&snapshot, script_id).health(),
        Health::Critical
    );

    let medication = medication_snapshot(&snapshot, "med01");
    assert_eq!(medication.health(), Health::Critical);
    assert_eq!(medication.remaining_supplies(), SupplyCount::ZERO);
}

#[test]
fn expired_script_is_critical() {
    let fixture = Fixture::new()
        .medication("med01")
        .expired_script("script01", &[("med01", 5)]);

    let snapshot = snapshot(&fixture, fixture.today());
    let script = script_snapshot(&snapshot, fixture.script_id("script01"));

    assert_eq!(script.health(), Health::Critical);
    let item = item_snapshot(script, "med01");
    assert_eq!(item.remaining_supplies(), SupplyCount::ZERO);
}

#[test]
fn expiring_script_is_attention() {
    let fixture = Fixture::new()
        .medication("med01")
        .expiring_script("script01", &[("med01", 3)]);

    let snapshot = snapshot(&fixture, fixture.today());
    let script = script_snapshot(&snapshot, fixture.script_id("script01"));
    assert!(matches!(script.health(), Health::Attention));
}

#[test]
fn script_issued_after_as_of_is_excluded() {
    let fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 2)]);

    let snapshot = snapshot(&fixture, fixture.yesterday());
    assert!(snapshot.scripts().is_empty());
    assert_eq!(
        medication_snapshot(&snapshot, "med01").health(),
        Health::Critical
    );
}

#[test]
fn multiple_medications_and_scripts_aggregate_correctly() {
    let fixture = Fixture::new();
    let today = fixture.today();

    let fixture = fixture
        .medication("med01")
        .medication("med02")
        .current_script("script01", &[("med01", 1), ("med02", 0)])
        .current_script("script02", &[("med01", 2)])
        .supply("supply01", "script01", "med01", today);

    let snapshot = snapshot(&fixture, fixture.today());

    let med01 = medication_snapshot(&snapshot, "med01");
    assert_eq!(med01.remaining_supplies(), SupplyCount::from(4));
    assert_eq!(med01.health(), Health::Ok);

    let med02 = medication_snapshot(&snapshot, "med02");
    assert_eq!(med02.remaining_supplies(), SupplyCount::ONE);
    assert_eq!(med02.health(), Health::Attention);

    assert_eq!(snapshot.scripts().len(), 2);
}

#[test]
fn script_with_all_items_exhausted_is_critical() {
    let fixture = Fixture::new();
    let today = fixture.today();

    let fixture = fixture
        .medication("med01")
        .medication("med02")
        .current_script("script01", &[("med01", 0), ("med02", 0)])
        .supply("a", "script01", "med01", today)
        .supply("c", "script01", "med02", today);

    let snapshot = snapshot(&fixture, today);
    let script = script_snapshot(&snapshot, fixture.script_id("script01"));

    assert_eq!(script.health(), Health::Critical);
    assert_eq!(item_snapshot(script, "med01").health(), Health::Critical);
    assert_eq!(item_snapshot(script, "med02").health(), Health::Critical);
}
