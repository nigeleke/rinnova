# -------------------------------------
# General text
date = {$day}/{$month}/{$year}
medication-definite = the medication
medication-indefinite = a medication
refill-definite = the refill
refill-indefinite = a refill
script-definite = the prescription
script-indefinite = a prescription
script-description =
    { $item_count ->
        *[one] from {$issued_on} to {$expires_on}; one item
        [other] from {$issued_on} to {$expires_on}; {$item_count} items
    }

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
    .duplicate-supply = Duplicated {$id}
    .script-out-of-date = Script {$id} is out of date
    .medication-not-on-script = Medication {$medication_id} is not on script {$script_id}
    .medication-out-of-refills = Medication {$medication_id} has no refills on script {$script_id}

# -------------------------------------
# Welcome Page
welcome-heading = rinnova

welcome-para-01 =
    rinnova helps keep track of medicines, prescriptions and repeats in one place.{" "}
    It makes it easier to see what is on hand, what is due next, and which prescription{" "}
    to use.

welcome-para-02 =
    All information is stored only on the device.{" "}
    rinnova is a personal organiser to help manage prescription records.{" "}
    It does not provide medical advice or replace guidance from a doctor or{" "}
    pharmacist.

welcome-continue-button =
    .text = Continue


# -------------------------------------
# Terms & Conditions Page
terms-heading = A few things first

terms-para-01 =
    All information is stored only on the device.{" "}
    rinnova is a personal organiser to help manage prescription records.{" "}
    It does not provide medical advice or replace guidance from a doctor or pharmacist.

terms-para-02 =
    Although every effort has been made to make rinnova reliable, no guarantee is{" "}
    given that the information entered or displayed is complete, accurate or up to date.{" "}
    Responsibility for checking medicines, prescriptions, repeat availability and expiry{" "}
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
remaining-supplies =
    { $n ->
        *[one] one remaining
        [other] {$n} remaining
    }
dispensed-button =
    .text = Dispensed
    .hint = Record selected medications dispensed
    .aria-label = Record selected medications dispensed
