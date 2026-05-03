.
├── agriculture
│   ├── application
│   │   ├── mod.rs
│   │   ├── ports
│   │   │   ├── cycle_repository.rs
│   │   │   ├── mod.rs
│   │   │   └── schedule_repository.rs
│   │   └── use_cases
│   │       ├── analyze_variance.rs
│   │       ├── mod.rs
│   │       └── schedule_crop_cycle.rs
│   ├── domain
│   │   ├── activity.rs
│   │   ├── analysis.rs
│   │   ├── area.rs
│   │   ├── crop.rs
│   │   ├── cycle.rs
│   │   ├── error.rs
│   │   ├── farm.rs
│   │   ├── mod.rs
│   │   ├── planning.rs
│   │   └── services
│   │       ├── economic_variance.rs
│   │       ├── mod.rs
│   │       ├── planning_service.rs
│   │       ├── variance.rs
│   │       └── variance_service.rs
│   ├── infrastructure
│   │   ├── mod.rs
│   │   └── repositories
│   │       └── mod.rs
│   ├── mod.rs
│   └── structure.md
├── finance
│   ├── application
│   ├── domain
│   │   ├── adapters
│   │   │   ├── agriculture_economic_provider.rs
│   │   │   └── mod.rs
│   │   ├── budget.rs
│   │   ├── expense.rs
│   │   └── mod.rs
│   ├── error.rs
│   ├── infrastructure
│   └── mod.rs
├── labor
│   ├── application
│   ├── domain
│   │   ├── mod.rs
│   │   ├── worker.rs
│   │   └── work_record.rs
│   ├── infrastructure
│   └── mod.rs
├── main.rs
├── shared_kernel
│   ├── ids.rs
│   ├── measurement
│   │   ├── area.rs
│   │   ├── error.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── money.rs
│   ├── space
│   │   ├── error.rs
│   │   ├── mod.rs
│   │   └── polygon.rs
│   └── time
│       ├── error.rs
│       ├── mod.rs
│       └── period.rs
└── tests
    ├── integration_test.rs
    └── mod.rs

23 directories, 51 files