# Create proxy website

## Create hosts file

Open a terminal

Edit the hosts file by running:
$ sudo nano /etc/hosts

## Using Nginx

Install Nginx (if not installed):
$ sudo apt-get install nginx

Configure Nginx:
Create or modify a configuration file in /etc/nginx/sites-available/ (you might need to create a new file specifically for your setup, like honeydragons.conf):
$ cd /etc/nginx/sites-available/

To create or edit the honeydragons configuration file using nano, run:
sudo nano [Name of the website]

Put code in created file:
server {
listen 80;
server_name [Name of the website.com];

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

}

Enable the Configuration:
Link your configuration file from sites-available to sites-enabled:
$ sudo ln -s /etc/nginx/sites-available/[Name of the website] /etc/nginx/sites-enabled/

2. Verify the Link
   After creating the symbolic link, check again to ensure it was created successfully:
   ls -l /etc/nginx/sites-enabled/
   You should now see honeydragons listed as a link to the file in sites-available.

3. Test Nginx Configuration
   Before restarting Nginx, it's always a good idea to test the configuration to ensure there are no syntax errors:
   $ sudo nginx -t
   This command should return "syntax is okay" and "test is successful".

4. Restart Nginx
   Once the link is verified and the configuration tests pass, restart Nginx to apply the changes:
   $ sudo systemctl restart nginx

5. Confirm Changes
   After restarting Nginx, you can confirm that [Name of the website.com] is being served by Nginx by using a curl command:
   curl -I http://[Name of the website].com
