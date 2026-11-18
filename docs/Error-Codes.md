# Error Codes

E000: Unknown or unspecified error, check logs for details


# E10x is Cache related errors

E101: Redis cache DB health check failed
E102: Redis cache Operation failed


# E11x is PostgreSQL related errors

E111: SQL Database Health check failed
E112: SQL Database error


# E12x is parsing data related errors

E121: Error on SerDe - Serialization/Deserialization failure
E122: Error on Lettre - Email address failure
E123: Random number generation error


# E13x is Lettre related errors

E131: Email sending error
E132: SMTP server error


# E14x is External Calls related errors

E141: External API call failed Upstream API
E142: Cloudflare Turnstile CAPTCHA verification failed
E143: Cloudflare Turnstile CAPTCHA Error response code received
E144: Failed to send notification to RabbitMQ by HTTP API call


# E2xx is Authorization/User related errors

## E20x is Password related errors

E201: Invalid base64 password
E202: Password must be between 8 and 64 characters
E203: Password must contain at least one uppercase letter
E204: Password must contain at least one lowercase letter
E205: Password must contain at least one digit
E206: Password must contain at least one special character

## E21x is Username related errors

E211: Username must be between 3 and 100 characters
E212: Username can only contain alphanumeric characters and underscores
E213: Username cannot start with a digit
E214: Username cannot contain consecutive underscores
E215: Username cannot start or end with an underscore
E216: Username cannot contain spaces
E217: Username can only contain ASCII characters

## E22x is Access Control related errors

E221: Insufficient permissions, user does not have the required set of permissions
E222: Insufficient tag access, user does not have access to the specified tag
