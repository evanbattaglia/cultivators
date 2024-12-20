# Step 1: login request `POST http://lti-13-test-tool.inst.test/login`

Form data (form-urlencoded, shown here as JSON for readability):

```json
{
  "iss": "https://canvas.instructure.com",
  "login_hint": "535fa085f22b4655f48cd5a36a9215f64c062838",
  "client_id": "10000000000360",
  "lti_deployment_id": "3407:8865aa05b4b79b64a91a86042e43af5ea8ae79eb",
  "target_link_uri": "http://lti-13-test-tool.inst.test/launch?placement=course_navigation",
  "lti_message_hint": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ2ZXJpZmllciI6ImIyMTFiMjIwMjYwZTg2ZjAwMGMyMDM3OTMzOGRhMWEzYjQ1NGM0NzMyZDM3M2E2MTM0MTc1YTA2MGMxMGIwYzFhNWVhMDkxYWJkZjdlOWMyZWQ1YmVhYTgxMTc5ZTBlZTg1NWNlOGI2NmY1M2QzMGU3NTcxMjhmODI0MjEyMTZmIiwiY2FudmFzX2RvbWFpbiI6ImNhbnZhcy13ZWIuaW5zdC50ZXN0IiwiY29udGV4dF90eXBlIjoiQ291cnNlIiwiY29udGV4dF9pZCI6MTAwMDAwMDAwMDAwMDEsImNhbnZhc19sb2NhbGUiOiJlbiIsImluY2x1ZGVfc3RvcmFnZV90YXJnZXQiOnRydWUsImV4cCI6MTczNDQ2ODU0N30.sL_K0tiGsN_0FmCLVlNG09b8UVtpbE-A9FbMV0NSXXU",
  "canvas_environment": "prod",
  "canvas_region": "not_configured",
  "lti_storage_target": "post_message_forwarding"
}
```

`lti_message_hint` JWT (TODO -- check spec to see if this is opaque to tool)
```
Token header
------------
{
  "typ": "JWT",
  "alg": "HS256"
}

Token claims
------------
{
  "canvas_domain": "canvas-web.inst.test",
  "canvas_locale": "en",
  "context_id": 10000000000001,
  "context_type": "Course",
  "exp": 1734468547,
  "include_storage_target": true,
  "verifier": "b211b220260e86f000c20379338da1a3b454c4732d373a6134175a060c10b0c1a5ea091abdf7e9c2ed5beaa81179e0ee855ce8b66f53d30e757128f82421216f"
}
```

# Step 2: authorize request `POST http://canvas-web.inst.test/api/lti/authorize_redirect`

Form data (form-urlencoded, shown here as JSON for readability):
```
{
  "utf8": "✓",
  "authenticity_token": "BALSNhu8PlOVr8piv3BSs7B8jn0hdusIsZy2j7qbqEgSBETMD6b00rCoZS1VEEdGhxQtbRIESoIcOkJsHJ1myA==",
  "scope": "openid",
  "response_type": "id_token",
  "client_id": "10000000000360",
  "redirect_uri": "http://lti-13-test-tool.inst.test/launch?placement=course_navigation",
  "login_hint": "535fa085f22b4655f48cd5a36a9215f64c062838",
  "lti_message_hint": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ2ZXJpZmllciI6ImIyMTFiMjIwMjYwZTg2ZjAwMGMyMDM3OTMzOGRhMWEzYjQ1NGM0NzMyZDM3M2E2MTM0MTc1YTA2MGMxMGIwYzFhNWVhMDkxYWJkZjdlOWMyZWQ1YmVhYTgxMTc5ZTBlZTg1NWNlOGI2NmY1M2QzMGU3NTcxMjhmODI0MjEyMTZmIiwiY2FudmFzX2RvbWFpbiI6ImNhbnZhcy13ZWIuaW5zdC50ZXN0IiwiY29udGV4dF90eXBlIjoiQ291cnNlIiwiY29udGV4dF9pZCI6MTAwMDAwMDAwMDAwMDEsImNhbnZhc19sb2NhbGUiOiJlbiIsImluY2x1ZGVfc3RvcmFnZV90YXJnZXQiOnRydWUsImV4cCI6MTczNDQ2ODU0N30.sL_K0tiGsN_0FmCLVlNG09b8UVtpbE-A9FbMV0NSXXU",
  "state": "bc4c03a1-329e-47ed-b255-410f44edf143",
  "response_mode": "form_post",
  "nonce": "7d5cde74-8ff2-4a44-8b41-3aab613c6f12",
  "prompt": "none"
}
```

# Step 3: launch request  `POST http://lti-13-test-tool.inst.test/launch?placement=course_navigation`
Form data (form-urlencoded, shown here as JSON for readability):
```
{
  "utf8": "✓",
  "authenticity_token": "3MdOALI+hnHr0Vn3yZoYxM44o8NtVpWW8wHbt+9MBhSoon5D213gJbiSa7mg1U+poXDIu1sjx/TBU7DToQstZw==",
  "id_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IjIwMjQtMTEtMDRUMTE6MDQ6MzlaXzE2YjVlOTVmLWRmZjgtNDcxNy1iZWRmLTY0ZjQwNzBiYzZjZiJ9.eyJodHRwczovL3B1cmwuaW1zZ2xvYmFsLm9yZy9zcGVjL2x0aS9jbGFpbS9tZXNzYWdlX3R5cGUiOiJMdGlSZXNvdXJjZUxpbmtSZXF1ZXN0IiwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGkvY2xhaW0vdmVyc2lvbiI6IjEuMy4wIiwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGkvY2xhaW0vcmVzb3VyY2VfbGluayI6eyJpZCI6IjRkZGUwNWU4Y2ExOTczYmNjYTliZmZjMTNlMTU0ODgyMGVlZTkzYTMiLCJkZXNjcmlwdGlvbiI6bnVsbCwidGl0bGUiOiJtYXRoLTEwMSJ9LCJhdWQiOiIxMDAwMDAwMDAwMDM2MCIsImF6cCI6IjEwMDAwMDAwMDAwMzYwIiwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGkvY2xhaW0vZGVwbG95bWVudF9pZCI6IjM0MDc6ODg2NWFhMDViNGI3OWI2NGE5MWE4NjA0MmU0M2FmNWVhOGFlNzllYiIsImV4cCI6MTczNDQ3MTg0NCwiaWF0IjoxNzM0NDY4MjQ0LCJpc3MiOiJodHRwczovL2NhbnZhcy5pbnN0cnVjdHVyZS5jb20iLCJub25jZSI6IjdkNWNkZTc0LThmZjItNGE0NC04YjQxLTNhYWI2MTNjNmYxMiIsInN1YiI6IjYyNWZmYTMwLWI1MmMtNDZjYy1iODdjLTY5ZmFjNzUyMDhjMCIsImh0dHBzOi8vcHVybC5pbXNnbG9iYWwub3JnL3NwZWMvbHRpL2NsYWltL3RhcmdldF9saW5rX3VyaSI6Imh0dHA6Ly9sdGktMTMtdGVzdC10b29sLmluc3QudGVzdC9sYXVuY2g_cGxhY2VtZW50PWNvdXJzZV9uYXZpZ2F0aW9uIiwicGljdHVyZSI6Imh0dHA6Ly9jYW52YXMuaW5zdHJ1Y3R1cmUuY29tL2ltYWdlcy9tZXNzYWdlcy9hdmF0YXItNTAucG5nIiwiZW1haWwiOiJlYmF0dGFnbGlhQGluc3RydWN0dXJlLmNvbSIsIm5hbWUiOiJlYmF0dGFnbGlhQGluc3RydWN0dXJlLmNvbSIsImdpdmVuX25hbWUiOiJlYmF0dGFnbGlhQGluc3RydWN0dXJlLmNvbSIsImZhbWlseV9uYW1lIjoiIiwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGkvY2xhaW0vbGlzIjp7InBlcnNvbl9zb3VyY2VkaWQiOiIkUGVyc29uLnNvdXJjZWRJZCIsImNvdXJzZV9vZmZlcmluZ19zb3VyY2VkaWQiOiIkQ291cnNlU2VjdGlvbi5zb3VyY2VkSWQifSwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGkvY2xhaW0vY29udGV4dCI6eyJpZCI6IjRkZGUwNWU4Y2ExOTczYmNjYTliZmZjMTNlMTU0ODgyMGVlZTkzYTMiLCJsYWJlbCI6Im1hdGgtMTAxIiwidGl0bGUiOiJtYXRoLTEwMSIsInR5cGUiOlsiaHR0cDovL3B1cmwuaW1zZ2xvYmFsLm9yZy92b2NhYi9saXMvdjIvY291cnNlI0NvdXJzZU9mZmVyaW5nIl19LCJodHRwczovL3B1cmwuaW1zZ2xvYmFsLm9yZy9zcGVjL2x0aS9jbGFpbS90b29sX3BsYXRmb3JtIjp7Imd1aWQiOiI3SGVrcko4UVZ6a29UR3dvUGE0RXpGVERRV2xaa1dDNHBEMXRyalcwOmNhbnZhcy1sbXMiLCJuYW1lIjoiVGhlIE5hbWUgT2YgWW91ciBPcmdhbml6YXRpb24iLCJ2ZXJzaW9uIjoiY2xvdWQiLCJwcm9kdWN0X2ZhbWlseV9jb2RlIjoiY2FudmFzIn0sImh0dHBzOi8vcHVybC5pbXNnbG9iYWwub3JnL3NwZWMvbHRpL2NsYWltL2xhdW5jaF9wcmVzZW50YXRpb24iOnsiZG9jdW1lbnRfdGFyZ2V0IjoiaWZyYW1lIiwicmV0dXJuX3VybCI6Imh0dHA6Ly9jYW52YXMtd2ViLmluc3QudGVzdC9jb3Vyc2VzLzEvZXh0ZXJuYWxfY29udGVudC9zdWNjZXNzL2V4dGVybmFsX3Rvb2xfcmVkaXJlY3QiLCJsb2NhbGUiOiJlbiIsImhlaWdodCI6NTAwLCJ3aWR0aCI6NTAwfSwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGkvY2xhaW0vcGxhdGZvcm1ub3RpZmljYXRpb25zZXJ2aWNlIjp7InNlcnZpY2VfdmVyc2lvbnMiOlsiMS4wIl0sInBsYXRmb3JtX25vdGlmaWNhdGlvbl9zZXJ2aWNlX3VybCI6Imh0dHA6Ly9jYW52YXMtd2ViLmluc3QudGVzdC9hcGkvbHRpL25vdGljZS1oYW5kbGVycy8zNDA3Iiwic2NvcGUiOlsiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGkvc2NvcGUvbm90aWNlaGFuZGxlcnMiXSwibm90aWNlX3R5cGVzX3N1cHBvcnRlZCI6WyJMdGlIZWxsb1dvcmxkTm90aWNlIiwiTHRpQXNzZXRQcm9jZXNzb3JTdWJtaXNzaW9uTm90aWNlIl19LCJsb2NhbGUiOiJlbiIsImh0dHBzOi8vcHVybC5pbXNnbG9iYWwub3JnL3NwZWMvbHRpL2NsYWltL3JvbGVzIjpbImh0dHA6Ly9wdXJsLmltc2dsb2JhbC5vcmcvdm9jYWIvbGlzL3YyL2luc3RpdHV0aW9uL3BlcnNvbiNBZG1pbmlzdHJhdG9yIiwiaHR0cDovL3B1cmwuaW1zZ2xvYmFsLm9yZy92b2NhYi9saXMvdjIvaW5zdGl0dXRpb24vcGVyc29uI1N0dWRlbnQiLCJodHRwOi8vcHVybC5pbXNnbG9iYWwub3JnL3ZvY2FiL2xpcy92Mi9zeXN0ZW0vcGVyc29uI1N5c0FkbWluIiwiaHR0cDovL3B1cmwuaW1zZ2xvYmFsLm9yZy92b2NhYi9saXMvdjIvc3lzdGVtL3BlcnNvbiNVc2VyIl0sImh0dHBzOi8vcHVybC5pbXNnbG9iYWwub3JnL3NwZWMvbHRpL2NsYWltL2N1c3RvbSI6eyJjYWxpcGVyX3VybCI6Imh0dHA6Ly9jYW52YXMtd2ViLmluc3QudGVzdC9hcGkvbHRpL3YxL2NhbGlwZXIvMzQwNy0xLTEtMTczNDQ2ODI0NC1kMGEzYWU0ZmEzZDgwMWUyLTQxODAwNzkyODc1OTRiZDNmNTQ5YjY5OGE5YmQwYWY1ZmIwZGMyZWQiLCJjYW52YXNfeGFwaV91cmwiOiJodHRwOi8vY2FudmFzLXdlYi5pbnN0LnRlc3QvYXBpL2x0aS92MS94YXBpLzM0MDctMS0xLTE3MzQ0NjgyNDQtNGNkZmE2MmQzNzBmZDQwNS0xMzdjNjUxODBhNTE5ZDg1OTliNWUzMDQ0ZjZlZDAzNWQwOTdiZGU0IiwiY2FudmFzX2FwaV9kb21haW4iOiJjYW52YXMtd2ViLmluc3QudGVzdCIsImNhbnZhc19hcGlfYmFzZXVybCI6Imh0dHA6Ly9jYW52YXMtd2ViLmluc3QudGVzdCIsImNhbnZhc19leHRlcm5hbHRvb2xfdXJsIjoiaHR0cDovL2NhbnZhcy13ZWIuaW5zdC50ZXN0L2FwaS92MS9hY2NvdW50cy8xL2V4dGVybmFsX3Rvb2xzLzM0MDciLCJ0b29scHJveHliaW5kaW5nX21lbWJlcnNoaXBzX3VybCI6Imh0dHA6Ly9jYW52YXMtd2ViLmluc3QudGVzdC9hcGkvbHRpL2NvdXJzZXMvMS9tZW1iZXJzaGlwX3NlcnZpY2UiLCJjYW52YXNfYXBpX2NvbGxhYm9yYXRpb25tZW1iZXJzX3VybCI6IiRDYW52YXMuYXBpLmNvbGxhYm9yYXRpb25NZW1iZXJzLnVybCJ9LCJodHRwczovL3B1cmwuaW1zZ2xvYmFsLm9yZy9zcGVjL2x0aS1hZ3MvY2xhaW0vZW5kcG9pbnQiOnsic2NvcGUiOlsiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGktYWdzL3Njb3BlL2xpbmVpdGVtIiwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGktYWdzL3Njb3BlL3Jlc3VsdC5yZWFkb25seSIsImh0dHBzOi8vcHVybC5pbXNnbG9iYWwub3JnL3NwZWMvbHRpLWFncy9zY29wZS9zY29yZSIsImh0dHBzOi8vY2FudmFzLmluc3RydWN0dXJlLmNvbS9sdGktYWdzL3Byb2dyZXNzL3Njb3BlL3Nob3ciXSwibGluZWl0ZW1zIjoiaHR0cDovL2NhbnZhcy13ZWIuaW5zdC50ZXN0L2FwaS9sdGkvY291cnNlcy8xL2xpbmVfaXRlbXMifSwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9sdGktbnJwcy9jbGFpbS9uYW1lc3JvbGVzZXJ2aWNlIjp7ImNvbnRleHRfbWVtYmVyc2hpcHNfdXJsIjoiaHR0cDovL2NhbnZhcy13ZWIuaW5zdC50ZXN0L2FwaS9sdGkvY291cnNlcy8xL25hbWVzX2FuZF9yb2xlcyIsInNlcnZpY2VfdmVyc2lvbnMiOlsiMi4wIl19LCJodHRwczovL3B1cmwuaW1zZ2xvYmFsLm9yZy9zcGVjL2x0aS9jbGFpbS9sdGkxMV9sZWdhY3lfdXNlcl9pZCI6IjUzNWZhMDg1ZjIyYjQ2NTVmNDhjZDVhMzZhOTIxNWY2NGMwNjI4MzgiLCJodHRwczovL3B1cmwuaW1zZ2xvYmFsLm9yZy9zcGVjL2x0aS9jbGFpbS9sdGkxcDEiOnsidXNlcl9pZCI6IjUzNWZhMDg1ZjIyYjQ2NTVmNDhjZDVhMzZhOTIxNWY2NGMwNjI4MzgifSwiaHR0cHM6Ly93d3cuaW5zdHJ1Y3R1cmUuY29tL3BsYWNlbWVudCI6ImNvdXJzZV9uYXZpZ2F0aW9uIn0.DXQAvo1S_9Qq_-2_TgBfXVX6NmoOBijw9TCUwVc6JTa2xYx80ZuZGBoRJ80KvdVQMhTBO7OdNPHCywtrqqJe-l7icZxJJLVLR0a_57rwf8VO9nLJOmdgN7tSl2G78w5zuSz8T0gT35J6P6DvnLUIncnOFkZNJoau9XNKlOvHUwuAdmZvh4H0mxYcedW1QbKfxXMN493g6s0Fr1pPWcoIqMidq63FkO1tX6felcxGC7heQNH-zqRjEogywDDNfUmc6E-ArTfqVa-aPM1xAgSxAdMFrdUHQa3pWN_LDZCGUWAe9tTOSuWtTFuoWusJGyO19OO9ngXYr07i-FJ3c652Ag",
  "state": "bc4c03a1-329e-47ed-b255-410f44edf143",
  "lti_storage_target": "post_message_forwarding"
}
```

`id_token` JWT

```
Token header
------------
{
  "typ": "JWT",
  "alg": "RS256",
  "kid": "2024-11-04T11:04:39Z_16b5e95f-dff8-4717-bedf-64f4070bc6cf"
}

Token claims
------------
{
  "aud": "10000000000360",
  "azp": "10000000000360",
  "email": "ebattaglia@instructure.com",
  "exp": 1734471844,
  "family_name": "",
  "given_name": "ebattaglia@instructure.com",
  "https://purl.imsglobal.org/spec/lti-ags/claim/endpoint": {
    "lineitems": "http://canvas-web.inst.test/api/lti/courses/1/line_items",
    "scope": [
      "https://purl.imsglobal.org/spec/lti-ags/scope/lineitem",
      "https://purl.imsglobal.org/spec/lti-ags/scope/result.readonly",
      "https://purl.imsglobal.org/spec/lti-ags/scope/score",
      "https://canvas.instructure.com/lti-ags/progress/scope/show"
    ]
  },
  "https://purl.imsglobal.org/spec/lti-nrps/claim/namesroleservice": {
    "context_memberships_url": "http://canvas-web.inst.test/api/lti/courses/1/names_and_roles",
    "service_versions": [
      "2.0"
    ]
  },
  "https://purl.imsglobal.org/spec/lti/claim/context": {
    "id": "4dde05e8ca1973bcca9bffc13e1548820eee93a3",
    "label": "math-101",
    "title": "math-101",
    "type": [
      "http://purl.imsglobal.org/vocab/lis/v2/course#CourseOffering"
    ]
  },
  "https://purl.imsglobal.org/spec/lti/claim/custom": {
    "caliper_url": "http://canvas-web.inst.test/api/lti/v1/caliper/3407-1-1-1734468244-d0a3ae4fa3d801e2-4180079287594bd3f549b698a9bd0af5fb0dc2ed",
    "canvas_api_baseurl": "http://canvas-web.inst.test",
    "canvas_api_collaborationmembers_url": "$Canvas.api.collaborationMembers.url",
    "canvas_api_domain": "canvas-web.inst.test",
    "canvas_externaltool_url": "http://canvas-web.inst.test/api/v1/accounts/1/external_tools/3407",
    "canvas_xapi_url": "http://canvas-web.inst.test/api/lti/v1/xapi/3407-1-1-1734468244-4cdfa62d370fd405-137c65180a519d8599b5e3044f6ed035d097bde4",
    "toolproxybinding_memberships_url": "http://canvas-web.inst.test/api/lti/courses/1/membership_service"
  },
  "https://purl.imsglobal.org/spec/lti/claim/deployment_id": "3407:8865aa05b4b79b64a91a86042e43af5ea8ae79eb",
  "https://purl.imsglobal.org/spec/lti/claim/launch_presentation": {
    "document_target": "iframe",
    "height": 500,
    "locale": "en",
    "return_url": "http://canvas-web.inst.test/courses/1/external_content/success/external_tool_redirect",
    "width": 500
  },
  "https://purl.imsglobal.org/spec/lti/claim/lis": {
    "course_offering_sourcedid": "$CourseSection.sourcedId",
    "person_sourcedid": "$Person.sourcedId"
  },
  "https://purl.imsglobal.org/spec/lti/claim/lti11_legacy_user_id": "535fa085f22b4655f48cd5a36a9215f64c062838",
  "https://purl.imsglobal.org/spec/lti/claim/lti1p1": {
    "user_id": "535fa085f22b4655f48cd5a36a9215f64c062838"
  },
  "https://purl.imsglobal.org/spec/lti/claim/message_type": "LtiResourceLinkRequest",
  "https://purl.imsglobal.org/spec/lti/claim/platformnotificationservice": {
    "notice_types_supported": [
      "LtiHelloWorldNotice",
      "LtiAssetProcessorSubmissionNotice"
    ],
    "platform_notification_service_url": "http://canvas-web.inst.test/api/lti/notice-handlers/3407",
    "scope": [
      "https://purl.imsglobal.org/spec/lti/scope/noticehandlers"
    ],
    "service_versions": [
      "1.0"
    ]
  },
  "https://purl.imsglobal.org/spec/lti/claim/resource_link": {
    "description": null,
    "id": "4dde05e8ca1973bcca9bffc13e1548820eee93a3",
    "title": "math-101"
  },
  "https://purl.imsglobal.org/spec/lti/claim/roles": [
    "http://purl.imsglobal.org/vocab/lis/v2/institution/person#Administrator",
    "http://purl.imsglobal.org/vocab/lis/v2/institution/person#Student",
    "http://purl.imsglobal.org/vocab/lis/v2/system/person#SysAdmin",
    "http://purl.imsglobal.org/vocab/lis/v2/system/person#User"
  ],
  "https://purl.imsglobal.org/spec/lti/claim/target_link_uri": "http://lti-13-test-tool.inst.test/launch?placement=course_navigation",
  "https://purl.imsglobal.org/spec/lti/claim/tool_platform": {
    "guid": "7HekrJ8QVzkoTGwoPa4EzFTDQWlZkWC4pD1trjW0:canvas-lms",
    "name": "The Name Of Your Organization",
    "product_family_code": "canvas",
    "version": "cloud"
  },
  "https://purl.imsglobal.org/spec/lti/claim/version": "1.3.0",
  "https://www.instructure.com/placement": "course_navigation",
  "iat": 1734468244,
  "iss": "https://canvas.instructure.com",
  "locale": "en",
  "name": "ebattaglia@instructure.com",
  "nonce": "7d5cde74-8ff2-4a44-8b41-3aab613c6f12",
  "picture": "http://canvas.instructure.com/images/messages/avatar-50.png",
  "sub": "625ffa30-b52c-46cc-b87c-69fac75208c0"
}
```
