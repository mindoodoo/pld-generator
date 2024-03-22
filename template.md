# Project Log Document

## Document Description

| **Title**       | **PLD Sprint Fast Forward**                                          |
| --------------- | -------------------------------------------------------------------- |
| **Subject**     | PLD Summarizing tasks to be performed during the fast forward sprint |
| **Authors**     | Nicolas Latteman & Léon Sautour                                      |
| **Supervisor**  | Lawal Alao                                                           |
| **Class Of**    | 2025                                                                 |
| **Update Date** | {{date_now}}                                                         |

## Table of Revisions

## Table of Contents

{{table_of_contents}}

## Diagram of the Deliverables

<center>
  ![](images/Autogrower%20-%20Sprint%204%20-%20Cards%20-%20General.png)

  ![](images/Autogrower%20-%20Sprint%204%20-%20Cards%20-%20Backend.png)

  ![](images/Autogrower%20-%20Sprint%204%20-%20Cards%20-%20Hardware.png)

  ![](images/Autogrower%20-%20Sprint%204%20-%20Cards%20-%20Frontend.png)

  ![](images/Autogrower%20-%20Sprint%204%20-%20Cards%20-%20Mobile.png)
</center>{{lucid}}

## Beta Plan

- Signin
- Signup
- Pairing a Device
- Looking at Device Data
- Looking at Plant Progress
- Calendar to view upcoming events
- Light/Dark Theme
- Settings page
  - Change Password
  - Change Email
  - Set preferred theme
  - Set preferred calendar view
- Unlink a Device
- Password Recovery
- Device usable to grow plant
- Device can grow different plants
- Display on Device to view data

## Cards

{{cards}}

## Progress

### Backend

- Implementation of the Device Event API
  - Featuring a portion targeting the user application and one targeting the devices.
  - Postman unit tests
- Backend has been redeployed on Railway due to Alex leaving the group
- Addition of third-party provider support for account creation and login
  - Google
  - Discord

### Hardware

- Adjustment of status LEDs to the electronics case
- Development of the improved nutrient dispenser
- Development of developer tooling to help visualize nutrient & ph measurement data
- Creation of a program to manually trigger nutrient and ph adjustment
- Updated Systemd units to work with the new program workflow
- Make use of Unix sockets for IPC, expose measurements to any program running on the device

### Frontend

- New frontend has been deployement on Vercel
- Addition of third-party providers on the login page
  - ![](images/login_page.png)
- Added email verification to the signup flow

### Mobile

- Decided for Flutter, since it is usable on IOS and Android
- Figma design finalized
 ![](images/figma_design.png)
- Development of the application's login/signup page started
  - UI completely implemented
