# Taskrunner

Taskrunner is a Rust application that generates an email based on the day of the week. It supports Gmail at the moment and will attach the corresponding file with the email, along with the contents of the file in the content section of the email. I am using this as a way of sending myself daily tasks. 

You can run this as a Cron Job each day to send yourself an email. I will also be building a feature that automatically executes this Cron Job from within the Rust application.

For this application to function, you will need to install Rust and set up a .env file with your gmail credentials. You will also need to allow your Gmail to view less secure apps <https://support.google.com/accounts/answer/6010255?hl=en>.

After you have installed Rust and Cargo locally, boot application with $ cargo run
