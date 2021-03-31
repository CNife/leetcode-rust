use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Default)]
pub struct Twitter {
    users: HashMap<UserId, Profile>,
    tweets: BTreeMap<Time, Tweet>,
    timer: u32,
}

pub type UserId = i32;
pub type TweetId = i32;
pub type Time = Reverse<u32>;

#[derive(Debug, Default)]
struct Profile {
    follows: HashSet<UserId>,
    tweets: Vec<TweetId>,
}

#[derive(Debug, Default)]
struct Tweet {
    id: TweetId,
    user: UserId,
}

impl Twitter {
    pub fn new() -> Twitter {
        Twitter::default()
    }

    pub fn post_tweet(&mut self, user: UserId, tweet: TweetId) {
        self.get_profile(user).tweets.push(tweet);
        self.tweets
            .insert(Reverse(self.timer), Tweet { id: tweet, user });
        self.timer += 1;
    }

    fn get_profile(&mut self, user: UserId) -> &mut Profile {
        self.users.entry(user).or_default()
    }

    pub fn get_news_feed(&self, user: UserId) -> Vec<TweetId> {
        match self.users.get(&user) {
            None => Vec::new(),
            Some(profile) => self
                .tweets
                .values()
                .filter(|tweet| tweet.user == user || profile.follows.contains(&tweet.user))
                .map(|tweet| tweet.id)
                .take(10)
                .collect(),
        }
    }

    pub fn follow(&mut self, follower: UserId, followee: UserId) {
        self.get_profile(follower).follows.insert(followee);
    }

    pub fn unfollow(&mut self, follower: UserId, followee: UserId) {
        self.get_profile(follower).follows.remove(&followee);
    }
}

#[test]
fn test() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
    twitter.unfollow(1, 2);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
}
