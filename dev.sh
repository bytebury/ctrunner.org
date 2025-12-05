# Check for the .env file; otherwise create one
if [ ! -f .env ]; then
  echo "🤖 .env not found. Generating..."
  cat >.env <<EOF
APP_NAME="CTRunner"
APP_WEBSITE_URL="http://localhost:8080" # or https://yourdomain.com
APP_PORT="8080"

DATABASE_URL="sqlite://db/database.db"

JWT_SECRET="SOMETHING-TOP-SECRET"

GOOGLE_CLIENT_ID="ADD_YOUR_CLIENT_ID"
GOOGLE_CLIENT_SECRET="ADD_YOUR_SECRET"
GOOGLE_CALLBACK_URL="http://localhost:8080/auth/google/callback"

COOKIE_URL="localhost:8080" # or .yourdomain.com

# THIS GOES TO A DUMMY FORM, CHANGE FOR PRODUCTION
SUBMIT_FORM_ID="1FAIpQLScHViJvQL0G_ZPuCZOIFNsBPthZwDSzbkgiFFeL93wp831diA"
SUBMIT_MEMBER_ID="1858653824"
SUBMIT_ACTION="517872474"
SUBMIT_FIRST_NAME="1421839249"
SUBMIT_LAST_NAME="390953767"
SUBMIT_TOWN_OF_RACE="1178659240"
SUBMIT_DATE_OF_RACE_YEAR="1640631443_year"
SUBMIT_DATE_OF_RACE_MONTH="1640631443_month"
SUBMIT_DATE_OF_RACE_DAY="1640631443_day"
SUBMIT_DISTANCE="1543094814"
SUBMIT_NAME_OF_RACE="1606581847"
SUBMIT_IS_169TH_TOWN="809023255"
SUBMIT_NOTIFY_OTHERS="1292315262"
SUBMIT_COMMENT="1729945787"

EOF
  echo "✅ .env generated."
else
  echo "✅ .env file found."
fi

# Check for the database file if it does not exist
if [ ! -f ./db/database.db ]; then
  echo "🤖 database not found. Generating..."
  mkdir ./db
  touch ./db/database.db
  echo "✅ database.db generated."
else
  echo "✅ database file found."
fi

# Start the development server with cargo watch
cargo watch -x run
