# Example Setup

### HTML Setup Example
- An example for how to set up a **<my-app-tables** in html:

``` html title="immunize/immunize.component.html"
<my-app-tables [alerts]="alerts" deletePreText="Do you wish to delete" deleteFieldDisplayField="@name"
  [enableView]="true" [enableDelete]="true" [enableAdd]="enableDelete" [formFilters]="formFilters" [args]="args"
  [listTypeUrl]="url" (actionsEvent)="handleActions($event)" [headers]="myHeaders" searchFieldName="name"
  searchFieldNameMessage="Name" [enableOrderBy]="enableOrderBy" (onStatusChange)="onDeleteStatus($event)">
</my-app-tables>
```

### TS Setup Example
- An example for how to set up a **multi form** in typescript:

```ts
import { Component, ElementRef, Input, OnInit, Output, ViewChild } from '@angular/core';
import { Router } from '@angular/router';
import { Chart, registerables, TooltipItem } from 'chart.js';


@Component({
  selector: 'app-immunize',
  templateUrl: './immunize.component.html',
  styleUrls: ['./immunize.component.scss']
})
export class ImmunizeComponent implements OnInit {

  @ViewChild("viewmodal") viewModalRef?: ElementRef
  @ViewChild("viewmodal2") viewModalRef2?: ElementRef

  title = "Administer Vaccine"
  subtitle = "View a list of all your patients"
  buttonText = "New Patient"
  buttonLink = "/patients/add"
  formFilters: any;





  constructor(private route: Router) {
    Chart.register(...registerables);
  }
  dataMap = {}
  alerts: any[] = [
    // {
    //   options: { level: "success" },
    //   message: "Mwangi Micha added",
    //   instance: {},
    // }
  ]
  ngOnInit(): void {
  }
  addAlert() {
    this.alerts.push(
      {
        options: { level: "info" },
        message: "Micha Kangi Logged in",
        instance: {},
      }
    )
  }

  days = [
    {
      value: "2020-01-01",
      males: 15328,
      females: 15170,
      total_students: 30498
    },
    {
      value: "2020-01-07",
      males: 12920,
      females: 12404,
      total_students: 25324
    },
    {
      value: "2020-01-13",
      males: 7579,
      females: 7568,
      total_students: 15147
    },
    {
      value: "2020-01-19",
      males: 5110,
      females: 5026,
      total_students: 10136
    },
    {
      value: "2020-02-04",
      males: 2559,
      females: 2510,
      total_students: 5069
    },
    {
      value: "2020-02-13",
      males: 15436,
      females: 15062,
      total_students: 30498
    },
    {
      value: "2020-02-19",
      males: 7729,
      females: 7418,
      total_students: 15147
    },
    {
      value: "2020-02-25",
      males: 7549,
      females: 7598,
      total_students: 15147
    },
    {
      value: "2020-03-04",
      males: 7599,
      females: 7548,
      total_students: 15147
    },
    {
      value: "2020-03-10",
      males: 5031,
      females: 5105,
      total_students: 10136
    },
    {
      value: "2020-03-16",
      males: 2533,
      females: 2536,
      total_students: 5069
    },
    {
      value: "2020-03-25",
      males: 15368,
      females: 15130,
      total_students: 30498
    },
    {
      value: "2020-04-04",
      males: 15258,
      females: 15240,
      total_students: 30498
    },
    {
      value: "2020-04-10",
      males: 10321,
      females: 9947,
      total_students: 20268
    }
  ]

  args = {}
  enableDelete = true;
  enableOrderBy = true;
  url: string = "api/v1/vaccines"
  instance: any;

  onFilters(filters: any) {
    this.formFilters = filters
  }

  isValidationOnly = true

  myHeaders = [
    {
      "source": "id",
      "name": "Patient ID"
    },
    {
      "source_interpolated": "@first_name @middle_name# @last_name#",
      "name": "Full Name"
    },
    {
      "name": "Created FUnc",
      "source_func": (val: any) => {
        var date = "@created#".interpolate(val).split("T")[0]
        return date.toDateDisplay()
      }

    },
    {
      "source": "dob",
      "name": "Date Of Birth"
    },
    {
      "source": "gender",
      "name": "Gender"
    },
    {
      "source": "guardian_name",
      "name": "Guardian Name"
    },
    {
      "source": "guardian_phone",
      "name": "Guardian Phone"
    },

  ]

  onDeleteStatus(instance: any) {
    if (typeof instance != "boolean") {
      this.alerts = [
        {
          options: { level: "danger" },
          message: "Vaccine  @name# deleted.",
          instance: instance,
        }
      ]
    }

  }

  async handleActions(event: any) {
    this.instance = null
    if (event.name == "Edit") {
      const data = event.data;
      console.log(data, 'customers')
      await this.route.navigate(['../customers/add'], { state: data });
    } else if (event.name == "Add") {
      const data = event.data;
      this.instance = data
      console.log(data, 'add');

      if (!this.viewModalRef) {
        console.log("NO button found")
        return
      }
      const viewModalButton = this.viewModalRef.nativeElement as HTMLButtonElement
      viewModalButton.click()
    } else if (event.name == "View") {
      const data = event.data;
      this.instance = data
      console.log(data, 'View');
      if (!this.viewModalRef2) {
        console.log("NO button found")
        return
      }
      const viewModalButton2 = this.viewModalRef2.nativeElement as HTMLButtonElement
      viewModalButton2.click()
    }
  }

}
```

### Final Output
- The final form will be served as follows:

<br>
<img src="../../../images/mytables/patientTable.png" height="600" width="1100">
<br>

**Useful Links**
<br>

- [Overview](../myForms/overview.md): Introduction to myForms.
- [Getting Started](../myForms/gettingStarted.md): Learn how to install and set up Sisitech Forms in your project.
- [Usage](../myForms/usage.md): Discover how to create, customize, and work with forms using our library.
- [Under the Hood](../myForms/underTheHood.md): Dive into the details of the library's functions, classes, and components.
- [Examples](../myForms/examples.md): See real-world examples of Sisitech Forms in action.
- [FAQs](../myForms/faqs.md): Find answers to common questions and troubleshooting tips.