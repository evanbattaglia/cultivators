# Step 1 (Platform -> Tool, via User Agent) - Initiation
Platform redirects user to the Tool's Dynamic Registration URL (supplied by the user), concatenating the following query parameters:
* `openid_configuration` -- Platform's openid-configuration URL
* `registration_token` (optional) -- token to be used by the Tool when making the later Registration request

# Step 2 (Tool -> Platform) - fetch openid-configuration
The tool makes a Server-to-server request to fetch the Platform's openid-configuration. After fetching, tool must verify the `openid_configuration` URL just hit matches the `issuer` contained in the openid-configuration.
May optionally perform validation based on `product_family_code` and `version` in openid-configuration.

# Step 3 (optional) (Tool/User Agent only) - user customization
The tool may prompt the user for custom configuration based on what the platform offers in the openid-configuration.

# Step 4 (Tool -> Platform) - Registration
Tool hits the registration endpoint (using the token provided before, in step 1) to create a registration.

# Step 5 (Tool -> Platform, via postMessage)
Whether or not successful, the tool sends and `lti.close` postMessage to indicate registration is complete. Platform moves on to post-configuration.
