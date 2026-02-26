# Instructions

## 1. You MUST NEVER use third party websites to figure out truth about tax forms. Only use https://www.irs.gov for it is the source of truth for tax matters.
## 2. When asked to research a tax form. Download the latest PDF first. Then read it and parse the details. Try to find the form in the `irs-form-schema` submodule if possible for it contains the official IRS MeF schema, that is, field names and data types. ALWAYS use the official IRS MeF schema if possible.
## 3. If something is a `group` in `irs-form-schema` that means it is a struct or `Vec`. Check the corresponding info in the PDF! Construct that struct or `Vec`.
## 4. Every field that shows up in `irs-form-schema` you must account for.