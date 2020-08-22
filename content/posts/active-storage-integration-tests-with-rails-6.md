+++
title = "Active Storage Integration Tests with Rails 6"
description = "Active Storage Integration Tests with Rails 6 for create and edit including removing the attachment"
date = 2019-10-17
+++

This article will cover creating integration tests using Rails 6 Active Storage. It will include the create and edit actions.

## Defining the Model and adding a Profile Picture

Scaffold out the model.

```bash
$ rails g scaffold Profile name:string
```
        
Update the model/profile.rb to include a profile picture.        

```ruby
class Profile < ApplicationRecord
    has_one_attached :profile_picture
end
```

Update the views/profiles/_form.html.erb to include a input for the profile picture.

```erb
<div class="field">
    <%= form.label :profile_picture %>
    <%= form.file_field :profile_picture %>
</div>
```

Update the controllers/profile_controller.rb to allow for the profile_picture params.

```ruby
def profile_params
    params.require(:profile).permit(:name, :profile_picture)
end
```

## Setting up Integration Tests

Add the integration test settings to test_helper.rb.

```ruby
require 'capybara/minitest'

class ActionDispatch::IntegrationTest
    # Make the Capybara DSL available in all integration tests
    include Capybara::DSL
    # Make `assert_*` methods behave like Minitest assertions
    include Capybara::Minitest::Assertions


    def setup
        Capybara.default_driver = :selenium_chrome_headless
        Capybara.default_max_wait_time = 5
    end

    # Reset sessions and driver between tests
    # Use super wherever this method is redefined in your individual test classes
    def teardown
        Capybara.reset_sessions!
        Capybara.use_default_driver
        Capybara.default_max_wait_time = 2
    end
end
```

## Create Profile Integration Test

Create a file called test/integration/profile_integration_test.rb and add the first test to create a profile. Remember to add a picture called picture_one.jpg to the test directory.

```ruby
# frozen_string_literal: true
require 'test_helper'

class ProfileIntegrationTest < ActionDispatch::IntegrationTest
    test 'that a profile can be created' do
        visit new_profile_path

        fill_in "Name", with: 'Jane Doe'

        find(:css, '#profile_profile_picture').set(File.join(Rails.root + "test", 'picture_one.jpg'))

        click_button 'Create Profile'

        assert_current_path profile_path(Profile.order("created_at").last)

        assert_equal ActiveStorage::Attachment.count, 1
    end
end
```

## Remove an Existing Profile Picture

Add a fixture to test/fixtures/profile.yml to represent the profile.

```yaml
remove_existing_profile_picture:
    name: 'Jane Doe'
```
            
We need to add an attribute to the model which we can use to determine if the profile picture should be removed or not.

```ruby
class Profile < ApplicationRecord
    has_one_attached :profile_picture


    attr_accessor :remove_existing_profile_picture
end
```

Update the controller to allow for the new parameter.

```ruby
def profile_params
    params.require(:profile).permit(:name, :profile_picture, :remove_existing_profile_picture)
end
```     

Update the form to show the remove profile picture checkbox, but only if a picture is attached.

```erb
<% if profile.profile_picture.attached? %>
    <%= form.label :remove_existing_profile_picture %>
    <%= form.check_box :remove_existing_profile_picture %>
<% end %>
```   

Update the controller to check for the remove_existing_profile_picture and remove the profile picture.
 
 ```ruby
def update
    if profile_params[:remove_existing_profile_picture] == "1"
      @profile_picture = @profile.profile_picture
      @profile_picture.purge_later
    end

    ...
end
```

Add the integration test to remove the profile picture.

 ```ruby
test 'that a profile picture can be removed ' do
    profile = profiles(:remove_existing_profile_picture)
    profile.profile_picture.attach(
        io: File.open(File.join(Rails.root + "test", 'picture_one.jpg')), 
        filename: 'picture_one.JPG', content_type: 'image/jpeg'
    )
    profile.save!

    visit edit_profile_path(profile)

    find(:css, "#profile_remove_existing_profile_picture").set(true)

    click_button 'Update Profile'

    assert_equal ActiveStorage::Attachment.count, 0
end
```

[Source](https://github.com/logankeenan/active-storage-integration-tests)