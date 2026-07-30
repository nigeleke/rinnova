mod fixture;
use fixture::Fixture;

// ------------------------------------
use scriptpilot::prelude::*;

#[test]
fn medication_can_be_added() {
    let fixture = Fixture::default().medication("med01");
    let id = fixture.medication_id("med01");
    assert!(fixture.has_medication(id));
}

#[test]
fn notionally_equivalent_medication_cannot_be_added() {
    let mut fixture = Fixture::default().medication("med01");
    let medication = Medication::new("MED01", "STRENGTH", "others notes");

    let result = fixture.logbook.try_add_medication(medication);
    assert!(matches!(result, Err(LogbookError::MatchingMedication(_))));
}

#[test]
fn existing_medication_can_be_removed() {
    let mut fixture = Fixture::default().medication("med01");
    let id = fixture.medication_id("med01");

    fixture
        .logbook
        .try_remove_medication(id)
        .expect("should remove medication");
    assert!(fixture.logbook.medications().is_empty());
}

#[test]
fn medication_can_be_readded_after_being_removed() {
    let mut fixture = Fixture::default().medication("med01");
    let id = fixture.medication_id("med01");

    fixture
        .logbook
        .try_remove_medication(id)
        .expect("medication should be removed");

    let replacement = Medication::new("med01", "strength", "notes");
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
    let mut fixture = Fixture::default().medication("med01");
    let id = fixture.medication_id("med01");

    let unknown = Medication::new("med01", "strength", "notes");
    let unknown_id = unknown.id();

    let result = fixture.logbook.try_remove_medication(unknown_id);
    assert!(matches!(result, Err(LogbookError::InvalidMedication(_))));

    assert!(fixture.has_medication(id));
    assert!(!fixture.has_medication(unknown_id));
}

#[test]
fn script_can_be_added() {
    let mut fixture = Fixture::default().medication("med01");

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
    let mut fixture = Fixture::default();

    let unknown_script_item = ScriptItem::new(MedicationId::new(), SupplyCount::from(2));

    let script = Script::try_new(Fixture::today(), Fixture::future(), &[unknown_script_item])
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
    let mut fixture = Fixture::default()
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
fn script_can_be_removed_if_all_supplies_used() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 2)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");

    assert!(fixture.logbook.try_remove_script(script_id).is_ok());
    assert!(fixture.logbook.scripts().is_empty());
}

#[test]
fn script_can_be_removed_even_if_current_and_supplies_remaining() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");

    assert!(fixture.logbook.try_remove_script(script_id).is_ok());
    assert!(fixture.logbook.scripts().is_empty());
}

#[test]
fn script_can_be_removed_if_expired_and_supplies_remaining() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .expired_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::past());

    let script_id = fixture.script_id("script01");

    assert!(fixture.logbook.try_remove_script(script_id).is_ok());
    assert!(fixture.logbook.scripts().is_empty());
}

#[test]
fn non_existing_script_cannot_be_removed() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);
    let script_id = fixture.script_id("script01");

    let non_existing_id = ScriptId::new();

    let result = fixture.logbook.try_remove_script(non_existing_id);
    assert!(matches!(result, Err(LogbookError::InvalidScript(_))));
    assert!(fixture.has_script(script_id));
}

#[test]
fn medication_not_referenced_by_any_script_can_be_removed() {
    let mut fixture = Fixture::default().medication("unused medication");
    let medication_id = fixture.medication_id("unused medication");

    fixture
        .logbook
        .try_remove_medication(medication_id)
        .expect("medication should be removed");

    assert!(!fixture.has_medication(medication_id));
}

#[test]
fn medication_in_script_cannot_be_removed() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);
    let medication_id = fixture.medication_id("med01");

    let result = fixture.logbook.try_remove_medication(medication_id);
    assert!(matches!(
        result,
        Err(LogbookError::MedicationUsedInScript(_))
    ));
    assert!(fixture.has_medication(medication_id));
}

#[test]
fn dispensing_can_be_recorded() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply("script01", "med01", Fixture::today());
    let supply_id = supply.id();

    fixture
        .logbook
        .record_supply(supply)
        .expect("dispensing should be recorded");

    assert!(fixture.has_supply(supply_id));
}

#[test]
fn dispensing_for_unknown_script_is_rejected() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply("unknown", "med01", Fixture::today());
    let supply_id = supply.id();

    let result = fixture.logbook.record_supply(supply);
    assert!(matches!(result, Err(LogbookError::InvalidScript(_))));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_for_unknown_medication_is_rejected() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply("script01", "unknown", Fixture::today());
    let supply_id = supply.id();

    let result = fixture.logbook.record_supply(supply);
    assert!(matches!(result, Err(LogbookError::InvalidMedication(_))));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_for_medication_not_on_script_is_rejected() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .medication("med02")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply("script01", "med02", Fixture::today());
    let supply_id = supply.id();

    let result = fixture.logbook.record_supply(supply);
    assert!(matches!(
        result,
        Err(LogbookError::MedicationNotOnScript(_, _))
    ));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_after_script_expiry_is_rejected() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .expired_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply("script01", "med01", Fixture::today());
    let supply_id = supply.id();

    let result = fixture.logbook.record_supply(supply);
    assert!(matches!(result, Err(LogbookError::ScriptOutOfDate(_))));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_before_script_issue_date_is_rejected() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply("script01", "med01", Fixture::yesterday());
    let supply_id = supply.id();

    let result = fixture.logbook.record_supply(supply);
    assert!(matches!(result, Err(LogbookError::ScriptOutOfDate(_))));

    assert!(!fixture.has_supply(supply_id));
}

#[test]
fn dispensing_can_occur_on_script_issue_date() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply("script01", "med01", Fixture::today());
    let supply_id = supply.id();

    fixture
        .logbook
        .record_supply(supply)
        .expect("dispensing should be recorded");

    assert!(fixture.has_supply(supply_id));
}

#[test]
fn dispensing_can_occur_on_script_expiry_date() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)]);

    let supply = fixture.build_supply("script01", "med01", Fixture::future());
    let supply_id = supply.id();

    fixture
        .logbook
        .record_supply(supply)
        .expect("dispensing should be recorded");

    assert!(fixture.has_supply(supply_id));
}

#[test]
fn dispensing_consumes_one_authorised_repeat() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 3)])
        .current_script("script02", &[("med01", 5)]);
    let script_id1 = fixture.script_id("script01");
    let script_id2 = fixture.script_id("script02");
    let medication_id = fixture.medication_id("med01");

    let supply = fixture.build_supply("script01", "med01", Fixture::today());

    let logbook = &mut fixture.logbook;

    assert_eq!(
        logbook.script_supply_count(script_id1, medication_id),
        SupplyCount::from(4)
    );
    assert_eq!(
        logbook.script_supply_count(script_id2, medication_id),
        SupplyCount::from(6)
    );
    assert_eq!(
        logbook.medication_supply_count(medication_id),
        SupplyCount::from(10)
    );

    logbook
        .record_supply(supply)
        .expect("initial supply should be recorded");

    assert_eq!(
        logbook.script_supply_count(script_id1, medication_id),
        SupplyCount::from(3)
    );
    assert_eq!(
        logbook.script_supply_count(script_id2, medication_id),
        SupplyCount::from(6)
    );
    assert_eq!(
        logbook.medication_supply_count(medication_id),
        SupplyCount::from(9)
    );
}

#[test]
fn dispensing_cannot_exceed_authorised_repeats() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 0)]);
    let script_id1 = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let supply01 = fixture.build_supply("script01", "med01", Fixture::today());
    let supply02 = fixture.build_supply("script01", "med01", Fixture::today());

    let logbook = &mut fixture.logbook;

    assert_eq!(
        logbook.script_supply_count(script_id1, medication_id),
        SupplyCount::ONE
    );
    assert_eq!(
        logbook.medication_supply_count(medication_id),
        SupplyCount::ONE
    );

    logbook
        .record_supply(supply01)
        .expect("initial supply should be recorded");

    assert_eq!(
        logbook.script_supply_count(script_id1, medication_id),
        SupplyCount::ZERO
    );
    assert_eq!(
        logbook.medication_supply_count(medication_id),
        SupplyCount::ZERO
    );

    let result = logbook.record_supply(supply02);
    assert!(matches!(
        result,
        Err(LogbookError::MedicationOutOfRefills(_, _))
    ))
}

#[test]
fn dispensing_is_allowed_while_authorised_repeats_remain() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 4)]);
    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let supplies = (0..5)
        .map(|_| fixture.build_supply("script01", "med01", Fixture::today()))
        .collect::<Vec<_>>();

    let logbook = &mut fixture.logbook;

    supplies
        .into_iter()
        .try_for_each(|supply| logbook.record_supply(supply))
        .expect("supplies should be recorded");

    assert_eq!(
        logbook.script_supply_count(script_id, medication_id),
        SupplyCount::ZERO
    );
    assert_eq!(
        logbook.medication_supply_count(medication_id),
        SupplyCount::ZERO
    );
}

#[test]
fn medication_can_be_dispensed_from_multiple_scripts() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 0)])
        .current_script("script02", &[("med01", 0)]);

    let script01_id = fixture.script_id("script01");
    let script02_id = fixture.script_id("script02");
    let medication_id = fixture.medication_id("med01");

    let supply01 = fixture.build_supply("script01", "med01", Fixture::today());
    let supply02 = fixture.build_supply("script02", "med01", Fixture::today());

    let logbook = &mut fixture.logbook;
    logbook
        .record_supply(supply01)
        .expect("supply should be recorded");
    logbook
        .record_supply(supply02)
        .expect("supply should be recorded");

    assert_eq!(
        logbook.script_supply_count(script01_id, medication_id),
        SupplyCount::ZERO
    );
    assert_eq!(
        logbook.script_supply_count(script02_id, medication_id),
        SupplyCount::ZERO
    );
    assert_eq!(
        logbook.medication_supply_count(medication_id),
        SupplyCount::ZERO
    );
}

#[test]
fn dispensing_from_one_script_does_not_affect_remaining_repeats_on_another_script() {
    let mut fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 0)])
        .current_script("script02", &[("med01", 0)]);

    let script01_id = fixture.script_id("script01");
    let script02_id = fixture.script_id("script02");
    let medication_id = fixture.medication_id("med01");

    let supply01 = fixture.build_supply("script01", "med01", Fixture::today());

    let logbook = &mut fixture.logbook;
    logbook
        .record_supply(supply01)
        .expect("supply should be recorded");

    assert_eq!(
        logbook.script_supply_count(script01_id, medication_id),
        SupplyCount::ZERO
    );
    assert_eq!(
        logbook.script_supply_count(script02_id, medication_id),
        SupplyCount::ONE
    );
    assert_eq!(
        logbook.medication_supply_count(medication_id),
        SupplyCount::ONE
    );
}

#[test]
fn current_script_reports_ok() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptOk(script_id)));
    assert!(status.contains(&Status::SupplyOk(script_id, medication_id)));
    assert!(status.contains(&&Status::MedicationOk(medication_id)));
    assert_eq!(status.len(), 3);
}

#[test]
fn script_due_to_expire_reports_due_to_expire() {
    let fixture = Fixture::default()
        .medication("med01")
        .expiring_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptDueToExpire(script_id)));
    assert!(status.contains(&Status::SupplyOk(script_id, medication_id)));
    assert!(status.contains(&Status::MedicationOk(medication_id)));
    assert_eq!(status.len(), 3);
}

#[test]
fn expired_script_reports_expired() {
    let fixture = Fixture::default()
        .medication("med01")
        .expired_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::past())
        .supply("supply02", "script01", "med01", Fixture::past());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptExpired(script_id)));
    assert!(status.contains(&Status::MedicationNotCovered(medication_id)));
    assert_eq!(status.len(), 2);
}

#[test]
fn medication_with_multiple_supplies_reports_ok() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptOk(script_id)));
    assert!(status.contains(&Status::SupplyOk(script_id, medication_id)));
    assert!(status.contains(&Status::MedicationOk(medication_id)));
    assert_eq!(status.len(), 3);
}

#[test]
fn medication_with_one_supply_remaining_reports_last_repeat() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 2)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptOk(script_id)));
    assert!(status.contains(&Status::LastRepeat(script_id, medication_id)));
    assert!(status.contains(&Status::MedicationOk(medication_id)));
    assert_eq!(status.len(), 3);
}

#[test]
fn medication_with_no_supplies_remaining_reports_no_repeats() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 2)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptExhausted(script_id)));
    assert!(status.contains(&Status::NoRepeats(script_id, medication_id)));
    assert!(status.contains(&Status::MedicationNotCovered(medication_id)));
    assert_eq!(status.len(), 3);
}

#[test]
fn medication_with_no_covering_script_reports_no_covering_script() {
    let fixture = Fixture::default().medication("med01");

    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::MedicationNotCovered(medication_id)));
    assert_eq!(status.len(), 1);
}

#[test]
fn medication_with_expired_script_reports_no_covering_script() {
    let fixture = Fixture::default()
        .medication("med01")
        .expired_script("script01", &[("med01", 5)]);

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptExpired(script_id)));
    assert!(status.contains(&Status::MedicationNotCovered(medication_id)));
    assert_eq!(status.len(), 2);
}

#[test]
fn medication_with_exhausted_script_reports_no_covering_script() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 1)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptExhausted(script_id)));
    assert!(status.contains(&Status::NoRepeats(script_id, medication_id)));
    assert!(status.contains(&Status::MedicationNotCovered(medication_id)));
    assert_eq!(status.len(), 3);
}

#[test]
fn due_to_expire_script_is_still_a_covering_script() {
    let fixture = Fixture::default()
        .medication("med01")
        .expiring_script("script01", &[("med01", 5)]);

    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::MedicationOk(medication_id)));
    assert!(!status.contains(&Status::MedicationNotCovered(medication_id)));
    assert_eq!(status.len(), 3);
}

#[test]
fn current_script_with_no_supplies_reports_no_repeats() {
    let fixture = Fixture::default()
        .medication("med01")
        .medication("med02")
        .current_script("script01", &[("med01", 1), ("med02", 2)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::NoRepeats(script_id, medication_id)));
}

#[test]
fn current_script_with_one_supply_remaining_reports_last_repeat() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 2)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::ScriptOk(script_id)));
    assert!(status.contains(&Status::LastRepeat(script_id, medication_id)));
    assert!(status.contains(&Status::MedicationOk(medication_id)));
    assert_eq!(status.len(), 3);
}

#[test]
fn medication_covered_by_multiple_scripts_reports_ok() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .current_script("script02", &[("med01", 3)]);

    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::MedicationOk(medication_id)));
    assert!(!status.contains(&&Status::MedicationNotCovered(medication_id)));
}

#[test]
fn medication_covered_by_current_and_expired_scripts_ignores_expired_script() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .expired_script("script02", &[("med01", 5)]);

    let medication_id = fixture.medication_id("med01");

    let status = fixture.logbook.evaluate_status(Fixture::today());

    assert!(status.contains(&Status::MedicationOk(medication_id)));
    assert!(!status.contains(&Status::MedicationNotCovered(medication_id)));
}

#[test]
fn medication_covered_by_multiple_scripts_sums_available_supplies() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .current_script("script02", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today())
        .supply("supply03", "script02", "med01", Fixture::today())
        .supply("supply04", "script02", "med01", Fixture::today())
        .supply("supply05", "script02", "med01", Fixture::today());

    let medication_id = fixture.medication_id("med01");

    assert_eq!(
        fixture.logbook.medication_supply_count(medication_id),
        SupplyCount::from(7)
    );
}

#[test]
fn script_health_is_ok_when_all_statuses_are_ok() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Ok
    );
}

#[test]
fn script_health_is_attention_when_script_due_to_expire() {
    let fixture = Fixture::default()
        .medication("med01")
        .expiring_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Attention
    );
}

#[test]
fn script_health_is_attention_when_last_repeat_remaining() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 2)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Attention
    );
}

#[test]
fn script_health_is_attention_when_due_to_expire_and_last_repeat() {
    let fixture = Fixture::default()
        .medication("med01")
        .expiring_script("script01", &[("med01", 3)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Attention
    );
}

#[test]
fn script_health_is_critical_when_script_expired() {
    let fixture = Fixture::default()
        .medication("med01")
        .expired_script("script01", &[("med01", 5)]);

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Critical
    );
}

#[test]
fn script_health_is_critical_when_no_repeats_remain() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 1)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Critical
    );
}

#[test]
fn script_health_is_critical_when_due_to_expire_and_no_repeats_remain() {
    let fixture = Fixture::default()
        .medication("med01")
        .expiring_script("script01", &[("med01", 1)])
        .supply("supply01", "script01", "med01", Fixture::today())
        .supply("supply02", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Critical
    );
}

#[test]
fn script_health_is_critical_when_expired_and_no_repeats_remain() {
    let fixture = Fixture::default()
        .medication("med01")
        .expired_script("script01", &[("med01", 2)])
        .supply("supply01", "script01", "med01", Fixture::past())
        .supply("supply02", "script01", "med01", Fixture::yesterday());

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Critical
    );
}

#[test]
fn script_health_ignores_other_scripts() {
    let fixture = Fixture::default()
        .medication("med01")
        .medication("med02")
        .current_script("script01", &[("med01", 5)])
        .expired_script("script02", &[("med02", 5)]);

    let script_id = fixture.script_id("script01");

    assert_eq!(
        fixture.logbook.script_health(script_id, Fixture::today()),
        Health::Ok
    );
}

#[test]
fn supply_health_is_ok_when_supply_is_available() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .supply("supply01", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    assert_eq!(
        fixture
            .logbook
            .supply_health(script_id, medication_id, Fixture::today()),
        Health::Ok
    );
}

#[test]
fn supply_health_is_attention_when_last_supply_remaining() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 1)])
        .supply("supply01", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    assert_eq!(
        fixture
            .logbook
            .supply_health(script_id, medication_id, Fixture::today()),
        Health::Attention
    );
}

#[test]
fn supply_health_is_critical_when_no_supplies_remaining() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 0)])
        .supply("supply01", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    assert_eq!(
        fixture
            .logbook
            .supply_health(script_id, medication_id, Fixture::today()),
        Health::Critical
    );
}

#[test]
fn supply_health_ignores_other_medications_on_same_script() {
    let fixture = Fixture::default()
        .medication("med01")
        .medication("med02")
        .current_script("script01", &[("med01", 1), ("med02", 5)])
        .supply("supply01", "script01", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med02");

    assert_eq!(
        fixture
            .logbook
            .supply_health(script_id, medication_id, Fixture::today()),
        Health::Ok
    );
}

#[test]
fn supply_health_ignores_other_scripts() {
    let fixture = Fixture::default()
        .medication("med01")
        .current_script("script01", &[("med01", 5)])
        .current_script("script02", &[("med01", 1)])
        .supply("supply01", "script02", "med01", Fixture::today());

    let script_id = fixture.script_id("script01");
    let medication_id = fixture.medication_id("med01");

    assert_eq!(
        fixture
            .logbook
            .supply_health(script_id, medication_id, Fixture::today()),
        Health::Ok
    );
}
