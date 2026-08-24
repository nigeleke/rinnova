use crate::domain::*;
use crate::test_support::Fixture;

#[test]
fn medication_can_be_added() {
    let fixture = Fixture::new().medication("med01");
    let id = fixture.medication_id("med01");
    assert!(fixture.has_medication(id));
}

#[test]
fn notionally_equivalent_medication_cannot_be_added() {
    let mut fixture = Fixture::new().medication("med01");
    let medication = Medication::try_new("MED01", "STRENGTH", "others notes")
        .expect("medication should be created");

    let result = fixture.logbook.try_add_medication(medication);
    assert!(matches!(result, Err(LogbookError::MatchingMedication(_))));
}

#[test]
fn existing_medication_can_be_removed() {
    let mut fixture = Fixture::new().medication("med01");
    let id = fixture.medication_id("med01");

    fixture
        .logbook
        .try_remove_medication(id)
        .expect("should remove medication");
    assert_eq!(fixture.logbook.medications().count(), 0);
}

#[test]
fn medication_can_be_readded_after_being_removed() {
    let mut fixture = Fixture::new().medication("med01");
    let id = fixture.medication_id("med01");

    fixture
        .logbook
        .try_remove_medication(id)
        .expect("medication should be removed");

    let replacement =
        Medication::try_new("med01", "strength", "notes").expect("medication should be created");
    let replacement_id = replacement.id();

    fixture
        .logbook
        .try_add_medication(replacement)
        .expect("medication should be added");

    assert!(!fixture.has_medication(id));
    assert!(fixture.has_medication(replacement_id));
}

#[test]
fn non_existing_medication_cannot_be_removed() {
    let mut fixture = Fixture::new().medication("med01");
    let id = fixture.medication_id("med01");

    let unknown =
        Medication::try_new("med01", "strength", "notes").expect("medication should be created");
    let unknown_id = unknown.id();

    let result = fixture.logbook.try_remove_medication(unknown_id);
    assert!(matches!(result, Err(LogbookError::MedicationNotFound(_))));

    assert!(fixture.has_medication(id));
    assert!(!fixture.has_medication(unknown_id));
}

#[test]
fn script_can_be_added() {
    let mut fixture = Fixture::new().medication("med01");

    let script = fixture.build_current_script(&[("med01", 5)]);
    let script_id = script.id();

    fixture
        .logbook
        .try_add_script(script)
        .expect("script should be added");

    assert!(fixture.has_script(script_id));
}

#[test]
fn script_for_unknown_medication_cannot_be_added() {
    let mut fixture = Fixture::new();

    let unknown_script_item = ScriptItem::new(MedicationId::new(), SupplyCount::from(2));

    let script = Script::try_new(fixture.today(), fixture.future(), &[unknown_script_item])
        .expect("script should be valid");

    let result = fixture.logbook.try_add_script(script);
    assert!(matches!(
        result,
        Err(LogbookError::UnknownMedication(id))
            if id == unknown_script_item.medication_id()
    ));
}

#[test]
fn existing_script_can_be_removed_if_expired() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .expired_script("script01", &[("med01", 5)]);
    let script_id = fixture.script_id("script01");

    fixture
        .logbook
        .try_remove_script(script_id)
        .expect("script should exist");

    assert!(!fixture.has_script(script_id));
}

#[test]
#[ignore]
fn script_can_be_removed_if_all_supplies_used() {
    let fixture = Fixture::new();
    let today = fixture.today();

    let mut fixture = fixture
        .medication("med01")
        .current_script("script01", &[("med01", 2)])
        .supply("supply01", "script01", "med01", today)
        .supply("supply02", "script01", "med01", today);

    let script_id = fixture.script_id("script01");

    assert!(fixture.logbook.try_remove_script(script_id).is_ok());
    assert_eq!(fixture.logbook.scripts().count(), 0);
}

#[test]
#[ignore]
fn script_can_be_removed_even_if_current_and_supplies_remaining() {
    let fixture = Fixture::new();
    let today = fixture.today();

    let mut fixture = fixture
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", today);

    let script_id = fixture.script_id("script01");

    assert!(fixture.logbook.try_remove_script(script_id).is_ok());
    assert_eq!(fixture.logbook.scripts().count(), 0);
}

#[test]
#[ignore]
fn script_can_be_removed_if_expired_and_supplies_remaining() {
    let fixture = Fixture::new();
    let past = fixture.past();

    let mut fixture = fixture
        .medication("med01")
        .expired_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", past);

    let script_id = fixture.script_id("script01");

    assert!(fixture.logbook.try_remove_script(script_id).is_ok());
    assert_eq!(fixture.logbook.scripts().count(), 0);
}

#[test]
fn non_existing_script_cannot_be_removed() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);
    let script_id = fixture.script_id("script01");

    let non_existing_id = ScriptId::new();

    let result = fixture.logbook.try_remove_script(non_existing_id);
    assert!(matches!(result, Err(LogbookError::ScriptNotFound(_))));
    assert!(fixture.has_script(script_id));
}

#[test]
fn medication_not_referenced_by_any_script_can_be_removed() {
    let mut fixture = Fixture::new().medication("unused medication");
    let medication_id = fixture.medication_id("unused medication");

    fixture
        .logbook
        .try_remove_medication(medication_id)
        .expect("medication should be removed");

    assert!(!fixture.has_medication(medication_id));
}

#[test]
fn medication_in_script_cannot_be_removed() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);
    let medication_id = fixture.medication_id("med01");

    let result = fixture.logbook.try_remove_medication(medication_id);
    assert!(matches!(result, Err(LogbookError::MedicationUsedInScript)));
    assert!(fixture.has_medication(medication_id));
}

#[test]
fn dispensing_can_be_recorded() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply(fixture.today(), &[("script01", "med01")]);
    let supply_id = supply.id();

    fixture
        .logbook
        .try_add_supply(supply)
        .expect("dispensing should be recorded");

    assert!(fixture.has_supply(supply_id));
}

#[test]
fn dispensing_for_unknown_script_is_rejected() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply(fixture.today(), &[("unknown", "med01")]);
    let supply_id = supply.id();

    let result = fixture.logbook.try_add_supply(supply);
    assert!(matches!(result, Err(LogbookError::ScriptNotFound(_))));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_for_unknown_medication_is_rejected() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply(fixture.today(), &[("script01", "unknown")]);
    let supply_id = supply.id();

    let result = fixture.logbook.try_add_supply(supply);
    assert!(matches!(result, Err(LogbookError::MedicationNotFound(_))));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_for_medication_not_on_script_is_rejected() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .medication("med02")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply(fixture.today(), &[("script01", "med02")]);
    let supply_id = supply.id();

    let result = fixture.logbook.try_add_supply(supply);
    assert!(matches!(
        result,
        Err(LogbookError::MedicationNotOnScript(_, _))
    ));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_after_script_expiry_is_rejected() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .expired_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply(fixture.today(), &[("script01", "med01")]);
    let supply_id = supply.id();

    let result = fixture.logbook.try_add_supply(supply);
    assert!(matches!(result, Err(LogbookError::ScriptOutOfDate(_))));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_before_script_issue_date_is_rejected() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply(fixture.yesterday(), &[("script01", "med01")]);
    let supply_id = supply.id();

    let result = fixture.logbook.try_add_supply(supply);
    assert!(matches!(result, Err(LogbookError::ScriptOutOfDate(_))));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_can_occur_on_script_issue_date() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply(fixture.today(), &[("script01", "med01")]);
    let supply_id = supply.id();

    fixture
        .logbook
        .try_add_supply(supply)
        .expect("dispensing should be recorded");

    assert!(fixture.has_supply(supply_id));
}

#[test]
fn dispensing_can_occur_on_script_expiry_date() {
    let mut fixture = Fixture::new()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply(fixture.future(), &[("script01", "med01")]);
    let supply_id = supply.id();

    fixture
        .logbook
        .try_add_supply(supply)
        .expect("dispensing should be recorded");

    assert!(fixture.has_supply(supply_id));
}
