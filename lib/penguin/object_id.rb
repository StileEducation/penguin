require_relative "../penguin_object_id"

class Penguin::ObjectId
  include Comparable

  ## Generate a new +ObjectId+ from the given time.
  #
  # Specifying `unique: true` will ensure the ID is unique for the given time.
  # Otherwise the ID may be the same if it is generated in the same second in
  # the same process.
  def self.from_time(time, unique: true)
    # `from_time` is defined in Ruby because it's a pain to work with kwargs in
    # Rust. Instead, call the native `generate_from_time` method which just uses
    # normal arguments.
    Penguin::ObjectId.generate_from_time(time, unique)
  end
end