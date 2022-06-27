INSTALLATION

Tested on:
    Host System: openSUSE Leap 15.3
    Docker: 20.10.12-ce
    Browser:
        Chrome: Version 101.0.4951.64 (Official Build) (64-bit)
        Firefox: 91.8.0esr (64-bit)
    VSCode: 1.67.2

OPTION 1:

1. Install "Remote - Containers" plugin in VSCode.
2. Open the "countries" folder in VSCode and it will prompt to reopen in container.
3. If not then from command pallete rebuild and reopen.

Note: It will also pull and run docker image for postgres and create a new network because initial version was using them.
      Currently these are not used. Need to delete them manually.

4. Once in remote container
    4.1 For backend

        cd backend
        cargo run  

        [api will serve on localhost:8080]

    4.2 For frontend

        cd frontend
        npm install
        npm run dev

        [frontend will be on localhost:3000]

5. Following are the valid field names for search:
    id, countryname, capital, population

6. Search query is case sensitive.
    
6. E.g. countryname:Nepal


OPTION 2:
1. Install rust v1.60.0 then same steps as above for backend
2. Install node v16.15.1 then same steps as above for frontend
