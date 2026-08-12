# -------------------------------------
# General text
version = Version {$version}
date = {$day}/{$month}/{$year}
medication-definite = the medication
medication-indefinite = a medication
medication-description-name = {$name}
medication-description-name-strength = {$name} ({$strength})
refill-definite = the refill
refill-indefinite = a refill
script-definite = the prescription
script-indefinite = a prescription
script-description = Date {$issued_on} - {$expires_on}
script-short-description = Script {$issued_on}
supply-definite = the supply
supply-indefinite = a supply
supply-description = Supply issued on {$issued_on}

# -------------------------------------
# Error messages
error =
    .invalid-date = Invalid date - {$error}
    .matching-medication = Medication {$name} previously added
    .duplicate-medication = Duplicated {$id}
    .invalid-medication = Invalid {$id}
    .invalid-draft-medication = Invalid draft medication
    .medication-used-in-script = Medication {$name} is referenced in scripts
    .invalid-expiry-date = Invalid expiry date {$date}
    .no-medications = Prescription has no medications
    .duplicate-script = Duplicated {$id}
    .invalid-script = Invalid {$id}
    .invalid-draft-script = Invalid draft script
    .unknown-medication = Unknown medication {$id}
    .invalid-supply = Invalid {$id}
    .duplicate-supply = Duplicated {$id}
    .script-out-of-date = Script {$id} is out of date
    .medication-not-on-script = Medication {$medication_id} is not on script {$script_id}
    .medication-out-of-refills = Medication {$medication_id} has no refills on script {$script_id}


# -------------------------------------
# Status messages
medication-status =
    .ok = Ok
    .last-repeat = Last repeat
    .no-repeats = No repeats

script-status =
    .ok = Ok
    .due-to-expire = Due to expire
    .not-current = Expired
    .exhausted = Exhausted

script-item-status =
    .ok = Ok
    .last-repeat = Last repeat
    .no-repeats = No repeats


# -------------------------------------
# Welcome Page
welcome-heading = rinnova

welcome-para-01 =
    rinnova helps keep track of prescriptions and repeats in one place.{" "}
    It makes it easier to see what is on hand, what is due next, and which prescription{" "}
    to use.

welcome-para-02 =
    All information is stored only on this device.{" "}
    rinnova is a personal organiser to help manage prescription records.{" "}
    It does not provide medical advice or replace guidance from a doctor or{" "}
    pharmacist.

welcome-continue-button =
    .text = Continue


# -------------------------------------
# Terms & Conditions Page
terms-heading = A few things first

terms-para-01 =
    All information is stored only on this device.{" "}
    rinnova is a personal organiser to help manage prescription records.{" "}
    It does not provide medical advice or replace guidance from a doctor or pharmacist.

terms-para-02 =
    Although every effort has been made to make rinnova reliable, no guarantee is{" "}
    given that the information entered or displayed is complete, accurate or up to date.{" "}
    Responsibility for checking medications, prescriptions, repeat availability and expiry{" "}
    dates remains with the user.{" "}
    The authors of rinnova accept no liability for any loss, damage or injury arising{" "}
    from its use.

terms-para-03 =
    rinnova stores only the information entered into the application.{" "}
    No information is transmitted to the authors or any third party.{" "}
    Responsibility for backing up device data rests with the user.{" "}
    Loss of data due to device failure, accidental deletion or other circumstances is{" "}
    the user's responsibility.

terms-para-04 =
    rinnova may be updated from time to time to add features, improve reliability{" "}
    or correct defects.{" "}
    Continued use of the application following an update constitutes acceptance of any{" "}
    revised terms.{" "}
    If any part of these terms is found to be unenforceable, the remaining provisions{" "}
    will continue to apply.

terms-confirmation-checkbox =
    .text = I understand and accept these terms & conditions.

terms-continue-button =
    .text = Continue


# -------------------------------------
# Header for each panel
panel-name =
    .reminders = Reminders
    .refills = Refills
    .scripts = Prescriptions
    .medications = Medications


# -------------------------------------
# Controls
ok-button =
    .text = Ok
    .aria-label = Submit changes

cancel-button =
    .text = Cancel
    .aria-label = Cancel changes

add-button =
    .text = Add
    .hint = Add {$indefinite_object}
    .aria-label = Click to add {$indefinite_object}

delete-button =
    .text = Delete
    .hint = Delete {$definite_object}
    .aria-label = Click to delete {$definite_object}

edit-button =
    .text = Edit
    .hint = Edit {$definite_object}
    .aria-label = Click to edit {$definite_object}


# -------------------------------------
# Medications panel
zero-medications-para-01 = Add current medications.
zero-medications-para-02 = These details can be changed later.
medication-form-medication-label = Medication
medication-form-strength-label = Strength (optional)
medication-form-notes-label = Notes (optional)
delete-medication = Permanently delete {$medication}?

# -------------------------------------
# Scripts panel
zero-scripts-para-01 = Add new prescriptions issued by your doctor.
zero-scripts-para-02 = These details can be changed later.
scripts-form-issued-on-label = Issued on:
scripts-form-expires-on-label = Expires on:
scripts-form-medication-heading = Medication
scripts-form-repeats-heading = Repeats
delete-script = Permanently delete {$script}?

# -------------------------------------
# Refills panel
dispensed-button =
    .text = Dispense
    .hint = Dispend selected medications
    .aria-label = Dispense selected medications
refill-form-issued-on-label = Issued on:

# -------------------------------------
# Reminders panel
reminders-subtitle =
    .no-repeats = No script
    .last-repeats = Last repeat
    .script-expiring = Script expiring
delete-supply = Permanently delete {$supply}?
