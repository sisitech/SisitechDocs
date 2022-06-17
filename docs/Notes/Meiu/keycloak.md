
# Drf Keycloak Intergration

## Update the keyclock server config

```ini title="api/settins/base.py"

OIDC_AUTH_URI = 'https://auth.peachcars.co.ke:8443/realms/Peach-dummy'
OIDC_CALLBACK_PUBLIC_URI = 'http://localhost:8000/'

LOGIN_REDIRECT_URL = OIDC_CALLBACK_PUBLIC_URI
LOGOUT_REDIRECT_URL = OIDC_AUTH_URI + '/protocol/openid-connect/logout?redirect_uri=' + OIDC_CALLBACK_PUBLIC_URI

OIDC_RP_CLIENT_ID = 'drfapi'
OIDC_RP_CLIENT_SECRET = 'guQmWhEWNylobr1GvgBSs7uP5cWIL7rM'
OIDC_RP_SCOPES = 'openid email profile role_list roles'

OIDC_RP_SIGN_ALGO='RS256'
# Keycloak-specific (as per http://KEYCLOAK_SERVER/auth/realms/REALM/.well-known/openid-configuration)
OIDC_OP_AUTHORIZATION_ENDPOINT = OIDC_AUTH_URI + '/protocol/openid-connect/auth'
OIDC_OP_TOKEN_ENDPOINT = OIDC_AUTH_URI + '/protocol/openid-connect/token'
OIDC_OP_USER_ENDPOINT = OIDC_AUTH_URI + '/protocol/openid-connect/userinfo'
OIDC_OP_JWKS_ENDPOINT = OIDC_AUTH_URI + '/protocol/openid-connect/certs'

LOGIN_URL="apiauth/login"

```

## Install mozilla-django-oidc

```bash
pip install mozilla-django-oidc
```

## Update api settins/base `installed_apps`

```python title="api/settins/base.py"
INSTALLED_APPS = [
    ...,
    'mozilla_django_oidc',
]
```

## Create a custom Authentication Backends

```python
from mozilla_django_oidc.auth import OIDCAuthenticationBackend

class KeycloakOIDCAuthenticationBackend(OIDCAuthenticationBackend):

    def create_user(self, claims):
        """ Overrides Authentication Backend so that Django users are
            created with the keycloak preferred_username.
            If nothing found matching the email, then try the username.
        """
        print("\n**** CREATE USER ****")
        print(claims)
        user = super(KeycloakOIDCAuthenticationBackend,
                     self).create_user(claims)
        user.first_name = claims.get('given_name', '')
        user.last_name = claims.get('family_name', '')
        user.email = claims.get('email')
        user.username = claims.get('preferred_username')
        user.save()
        return user

    def filter_users_by_claims(self, claims):
        """ Return all users matching the specified email.
            If nothing found matching the email, then try the username
        """
        print("\n**** FILTER USER ****")
        print(claims)
        email = claims.get('email')
        preferred_username = claims.get('preferred_username')
        if not email:
            return self.UserModel.objects.none()
        users = self.UserModel.objects.filter(email__iexact=email)

        if len(users) < 1:
            if not preferred_username:
                return self.UserModel.objects.none()
            users = self.UserModel.objects.filter(
                username__iexact=preferred_username)
        return users

    def update_user(self, user, claims):
        print("\n**** UPDATE USER ****")
        print(claims)
        user.first_name = claims.get('given_name', '')
        user.last_name = claims.get('family_name', '')
        user.email = claims.get('email')
        user.username = claims.get('preferred_username')
        user.save()
        return user

```

## Set the dafault authenticatoin backend

```python title="api/settins/base.py"
AUTHENTICATION_BACKENDS = (
    'django.contrib.auth.backends.ModelBackend',
    'api.auth.KeycloakOIDCAuthenticationBackend',
)
```

## Set the custom authentication backend

```python title="api/settins/base.py"
OIDC_DRF_AUTH_BACKEND = 'api.auth.KeycloakOIDCAuthenticationBackend'

```

## Update Rest Framework default authentication classes

```python title="api/settins/base.py"
REST_FRAMEWORK = {
    'DEFAULT_AUTHENTICATION_CLASSES': (
        'rest_framework.authentication.SessionAuthentication',
        'mozilla_django_oidc.contrib.drf.OIDCAuthentication',
    ),
    'DEFAULT_PAGINATION_CLASS': 'mylib.my_common.MyStandardPagination',
    'PAGE_SIZE': 100

}
```
