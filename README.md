## Motivation
The inspiration for this repo came from using Github discussions to track community ideas for one of our TApps. That got us thinking: why not use a TApp itself to keep track of these ideas? Community members could vote for app ideas they like by pledging some of their TEA. This would make voting more meaningful as community members positive sentiment on promising apps would be backed by their pledged TEA.

This type of voting app is common in web2 but gives us a good opportunity to discuss what has to change as we move into developing on web3. The TApp in this repo implements the following features:

All community members can post ideas about what TApps they want to build. For every TApp idea they post they’ll need to put up at least 1 T as an initial deposit for each idea.
Community members vote to support an idea by locking as collateral for their vote of at least 1T. All T collateral that’s put up for votes will be added to the initial deposit.
Once a user has voted, they cannot withdraw their T collateral that they used for voting. But they can increase their collateral deposit by voting again and adding at least 1T to their existing voting pledge.

The ideas are ordered by the total deposit count. The higher the vote total, as measured by the total vote collateral (including the initial deposit), the higher its position will be in the app.
Note that there's currently no worker feature for users to complete the voted-on idea as a task. This idea could be added by us in the future but is also a great opportunity for a developer to add their own feature set on top of the idea voting TApp.
